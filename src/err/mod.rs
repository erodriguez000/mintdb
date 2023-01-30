#![allow(dead_code, unused)]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("The request body contains invalid data")]
	Request,

	#[error("There was no NS header present in the request")]
	NoNsHeader,

	#[error("There was no DB header present in the request")]
	NoDbHeader,

	#[error("There was a problem with authentication")]
	InvalidAuth,

	#[error("The specified media type is unsupported")]
	InvalidType,

	#[error("There was a problem connecting with the storage engine")]
	InvalidStorage,

	#[error("There was a problem fetching data")]
    ErrorFetching,

	#[error("The table {0} alread exists")]
    ErrorAddingTable(String),

    #[error("Table was not found: {0}")]
    TableNotFound(String),

    #[error("Document was not found: {0}")]
    DocumentNotFound(String),

	#[error("The document {0} already exists")]
    DocumentExists(String),

	#[error("Key was not found: {0}")]
    KeyNotFound(String),

    #[error("There was an issue parsing values to publish")]
    PublishingParsingError(#[from] serde_json::Error),

	#[error("There was an issue parsing values to publish")]
    RequestError(#[from] mintdb::err::Error),

	#[error("There was an issue parsing values to publish")]
    IOError(std::io::Error),
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}
