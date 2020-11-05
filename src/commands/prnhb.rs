use crate::{
    StandardResult,
    error::CatBoatError
};
use hyper::{
    Body,
    Client,
    body,
    Request,
    client::HttpConnector
};
use hyper_tls::HttpsConnector;
use select::{
    document::Document,
    predicate::{Attr, Class, Name}
};

#[allow(dead_code)]
const URL_BASE: &str = "https://fr.pornhub.com";

#[derive(Debug)]
struct Video {
    pub title: String,
    pub href: String,
    pub img: String
}

#[allow(dead_code)]
async fn search<'a>(client: &Client<HttpsConnector<HttpConnector>>, query: &str) -> StandardResult<Vec<Video>> {
    let req = Request::builder()
        .method("GET")
        .uri([URL_BASE, "/video/search?search=", query].concat())
        .header("Cookie", "platform=pc;accessAgeDisclaimerPH=1")
        .header("User-Agent", "CatBoat/1.0.0")
        .body(Body::empty())?;

    let res = client.request(req).await?;
    let document = Document::from(&(match res.status().as_u16() {
        200 => Ok(String::from_utf8(body::to_bytes(res).await?.to_vec())?),
        _ => Err(CatBoatError::Unknown),
    })?[..]);

    let videos_selection = document
        .find(Attr("id", "videoSearchResult"))
        .into_selection()
        .children()
        .filter(Class("pcVideoListItem"))
        .find(Name("a"))
        .filter(Class("linkVideoThumb"));

    Ok(videos_selection.iter().map(|video| -> StandardResult<Video> {
        Ok(Video {
            title: video.attr("title").ok_or(CatBoatError::SelectAttrNotFound)?.to_owned(),
            href: [URL_BASE, &video.attr("href").ok_or(CatBoatError::SelectAttrNotFound)?.to_owned()].concat(),
            img: video.find(Name("img"))
                .next().ok_or(CatBoatError::Unknown)?
                .attr("data-thumb_url").ok_or(CatBoatError::SelectAttrNotFound)?.to_owned(),
        })
    }).collect::<StandardResult<Vec<Video>>>()?)
}

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
async fn search_p(ctx: &Context, msg: &Message) -> CommandResult {
    let videos = search(&crate::HTTPS_CLIENT, "test").await.expect("Error while searching");

    for vid in videos {
        msg.channel_id.say(&ctx.http, format!("Title: {}\nURL: {}", vid.title, vid.href)).await?;
    }

    Ok(())
}