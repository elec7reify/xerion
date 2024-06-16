use reqwest::header::InvalidHeaderValue;
use reqwest::Error;
use std::error::Error as Err;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ClientError {
    InvalidHeader(InvalidHeaderValue),
    Request(Error),
}

impl From<InvalidHeaderValue> for ClientError {
    fn from(error: InvalidHeaderValue) -> Self {
        Self::InvalidHeader(error)
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHeader(_) => f.write_str("Invalid auth"),
            Self::Request(_) => f.write_str("Error while sending HTTP request."),
        }
    }
}

impl Err for ClientError {
    fn source(&self) -> Option<&(dyn Err + 'static)> {
        match self {
            _ => None,
        }
    }
}
