use crate::{error::CatBoatError, HttpClient, StandardResult, BOT_PREFIX};
use hyper::{body, client::HttpConnector, Body, Client, Request, Response};
use hyper_tls::HttpsConnector;
use select::{
    document::Document,
    predicate::{Attr, Class, Name},
};
use urlencoding::encode;

#[allow(dead_code)]
const URL_BASE: &str = "https://fr.pornhub.com";

#[derive(Debug)]
struct Video {
    pub title: String,
    pub href: String,
    pub img: String,
}

use tracing::warn;

async fn get(
    client: &Client<HttpsConnector<HttpConnector>>,
    uri: &str,
) -> StandardResult<Response<Body>> {
    Ok(client
        .request(
            Request::builder()
                .method("GET")
                .uri(uri)
                .header("Cookie", "platform=pc;accessAgeDisclaimerPH=1")
                .header("User-Agent", "CatBoat/1.0.0")
                .body(Body::empty())?,
        )
        .await?)
}

async fn search<'a>(
    client: &Client<HttpsConnector<HttpConnector>>,
    query: &str,
) -> StandardResult<Vec<Video>> {
    warn!(
        "{}",
        [URL_BASE, "/video/search?search=", &encode(query)].concat()
    );

    let res = get(
        client,
        &[
            URL_BASE,
            "/video/search?search=",
            &encode(query).replace("%20", "+"),
        ]
        .concat(),
    )
    .await?;

    let document = Document::from(
        &(match res.status().as_u16() {
            200 => Ok(String::from_utf8(body::to_bytes(res).await?.to_vec())?),
            // If redirect => try to recover and make another request
            status @ 301..=302 => {
                if let Some(url_part) = &res.headers().get("LOCATION") {
                    let res = get(client, &[URL_BASE, url_part.to_str()?].concat()).await?;

                    Ok(match res.status().as_u16() {
                        200 => Ok(String::from_utf8(body::to_bytes(res).await?.to_vec())?),
                        status => Err(CatBoatError::BadHyperResponse { status }),
                    }?)
                } else {
                    Err(CatBoatError::BadHyperResponse { status })
                }
            }
            status => Err(CatBoatError::BadHyperResponse { status }),
        })?[..],
    );

    let videos_selection = document
        .find(Attr("id", "videoSearchResult"))
        .into_selection()
        .children()
        .filter(Class("pcVideoListItem"))
        .find(Name("a"))
        .filter(Class("linkVideoThumb"));

    Ok(videos_selection
        .iter()
        .map(|video| -> StandardResult<Video> {
            Ok(Video {
                title: video
                    .attr("title")
                    .ok_or(CatBoatError::SelectAttrNotFound)?
                    .to_owned(),
                href: [
                    URL_BASE,
                    &video
                        .attr("href")
                        .ok_or(CatBoatError::SelectAttrNotFound)?
                        .to_owned(),
                ]
                .concat(),
                img: video
                    .find(Name("img"))
                    .next()
                    .ok_or(CatBoatError::ImageNotFound)?
                    .attr("data-thumb_url")
                    .ok_or(CatBoatError::SelectAttrNotFound)?
                    .to_owned(),
            })
        })
        .collect::<StandardResult<Vec<Video>>>()?)
}

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

const COMMAND: &str = "phub";

#[command]
pub async fn phub(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let http_client = data
        .get::<HttpClient>()
        .expect("Expected Hyper https client in TypeMap.");

    let mut msg_content = (&msg.content.split(' ').collect::<Vec<&str>>()).clone();

    if msg_content.len() <= 1 {
        msg.channel_id
            .say(
                &ctx.http,
                MessageBuilder::new()
                    .mention(&msg.author)
                    .push(" Merci d'utiliser la commande comme ceci : ")
                    .push_mono([BOT_PREFIX, COMMAND, " votre recherche"].concat())
                    .build(),
            )
            .await?;
    } else {
        msg_content.remove(0);
        let query = msg_content.join(" ");
        warn!("{}", query);

        if let Some(vid) = search(http_client, &query)
            .await
            .expect("Error while searching")
            .get(0)
        {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.content(["*Search: ", &query, "*"].concat());
                    m.embed(|e| {
                        e.title(&vid.title);
                        e.description(&vid.href);
                        e.url(&vid.href);
                        e.image(&vid.img);
                        e
                    });
                    m
                })
                .await?;
        }
    }

    Ok(())
}
