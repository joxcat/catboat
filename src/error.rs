use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum CatBoatError {
    #[error("UTF-8 error")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Hyper error")]
    HyperError(#[from] hyper::Error),
    #[error("Hyper http error")]
    HyperHttpError(#[from] hyper::http::Error),
    #[error("Hyper ToStr ERROR")]
    HyperToStrError(#[from] hyper::header::ToStrError),
    #[error("Attribute not found in Select")]
    SelectAttrNotFound,
    #[error("Image not found in Select")]
    ImageNotFound,
    #[error("Not handled status code {{.status}}")]
    BadHyperResponse { status: u16 },
    #[error("Unknown bot error")]
    Unknown,
}
