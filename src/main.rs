use anyhow::{Error, Context};
use hyper::Client;
use hyper_tls::HttpsConnector;

mod error;
mod apis;

pub type StandardResult<T> = Result<T, Error>;

#[tokio::main]
async fn main() -> StandardResult<()> {
	let https = HttpsConnector::new();
	let client = Client::builder().build::<_, hyper::Body>(https);

	let vids = apis::prnhb::search(&client, "test").await.context("Error while searching videos")?;

	for vid in vids {
		println!("{:?}", vid);
	}

	Ok(())
}
