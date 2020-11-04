use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatBoatError {
	#[error("unknown bot error")]
	Unknown,
}