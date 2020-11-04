use anyhow::Error;
use hyper::{Client, body, Request, Body};
use hyper_tls::HttpsConnector;

mod error;

pub type StandardResult<T> = Result<T, Error>;

#[tokio::main]
async fn main() -> StandardResult<()> {
	let https = HttpsConnector::new();
	let client = Client::builder().build::<_, hyper::Body>(https);
	let req = Request::builder()
		.method("GET")
		.uri("https://fr.pornhub.com/video/search?search=sfw")
		.header("Cookie", "platform=pc;accessAgeDisclaimerPH=1")
		.header("User-Agent", "CatBoat/1.0.0")
		.body(Body::empty())?;

	let res = client.request(req).await?;
	//let body = String::from_utf8(body::to_bytes(res).await?.to_vec())?;
	println!("{:?}", res);
	Ok(())
}
