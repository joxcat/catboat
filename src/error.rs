use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum CatBoatError {
	#[error("Attribute not found in Select")]
	SelectAttrNotFound,
	#[error("Unknown bot error")]
	Unknown,
}