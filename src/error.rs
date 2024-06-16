use crate::ClientError;
use reqwest::header::InvalidHeaderValue;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Url(url::ParseError),
    Http(ClientError),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

impl From<ClientError> for Error {
    fn from(error: ClientError) -> Self {
        Error::Http(error)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(error: InvalidHeaderValue) -> Self {
        ClientError::InvalidHeader(error).into()
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        ClientError::Request(e).into()
    }
}
