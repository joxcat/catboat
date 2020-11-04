use crate::StandardResult;
use crate::error::CatBoatError;
use hyper::{Body, Client, body, Request};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use select::document::Document;
use select::predicate::{Attr, Class, Name};

#[derive(Debug)]
pub struct Video {
    pub title: String,
    pub href: String,
    pub img: String
}

pub async fn search<'a>(client: &Client<HttpsConnector<HttpConnector>>, query: &str) -> StandardResult<Vec<Video>> {
    let req = Request::builder()
        .method("GET")
        .uri(["https://fr.pornhub.com/video/search?search=", query].concat())
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
            href: video.attr("href").ok_or(CatBoatError::SelectAttrNotFound)?.to_owned(),
            img: video.find(Name("img"))
                .next().ok_or(CatBoatError::Unknown)?
                .attr("data-thumb_url").ok_or(CatBoatError::SelectAttrNotFound)?.to_owned(),
        })
    }).collect::<StandardResult<Vec<Video>>>()?)
}