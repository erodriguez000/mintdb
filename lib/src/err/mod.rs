use thiserror::Error;
#[derive(Error, Debug)]
#[allow(unused)]
pub enum Error {
    #[error("The request body contains invalid data")]
	Request,

    #[error("Missing '{0}' field in request body")]
	RequestMissingKey(String),

    #[error("Transaction finished")]
	TxFinished,

    #[error("IO Error")]
	IOError(#[from] std::io::Error),

    #[error("Bin Error")]
	BinError(#[from] serde_cbor::Error),

    #[error("Table '{0}' does not exist")]
    TableNotFound(String),
    
    #[error("Document '{0}' does not exist")]
    DocumentNotFound(String),

    #[error("Key '{0}' not found on Document '{1}'")]
    KeyNotFound(String, String),

    #[error("Document '{0}' already exists")]
    TableExists(String),

    #[error("Document '{0}' already exists")]
    DocumentExists(String),

    #[error("Key '{0}' exists on Document '{1}'")]
    KeyExists(String, String),

    #[error("Key '{0}' contains an invalid type for this operation")]
    KeyContainsInvalidType(String),    

    #[error("There was an issue parsing values to publish")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("Invalid credentials provided")]
    InvalidCredentials(#[from] ring::error::Unspecified),

    #[error("Invalid type provided")]
    InvalidType(#[from] serde_json::error::Error),

    #[error("The API token provided is invalid")]
	InvalidAPIToken,

    #[error("The API token provided has expired")]
	InvalidAPITokenExp,

    #[error("Schema missing Key '{0}', of type '{1}'")]
	SchemaMissingKeyofType(String, String),

    #[error("Schema invalid Key '{0}', for table '{1}'")]
	SchemaInvalidKeyTable(String, String),

    #[error("Invalid data type for Key '{0}', type must be '{1}")]
	SchemaInvalidDataType(String, String),
}