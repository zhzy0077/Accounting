use actix_web::error;
use actix_web::http::StatusCode;
use std::fmt::Formatter;
use std::{fmt, io};

#[derive(Debug)]
pub enum ServerError {
    DBError(String),
    IOError,
    InternalError(String),
    UnauthorizedError,
}

impl error::ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::UnauthorizedError => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for ServerError {
    fn from(_: io::Error) -> Self {
        ServerError::IOError
    }
}

impl From<r2d2::Error> for ServerError {
    fn from(_: r2d2::Error) -> Self {
        ServerError::IOError
    }
}

impl From<rusqlite::Error> for ServerError {
    fn from(_: rusqlite::Error) -> Self {
        ServerError::IOError
    }
}

impl From<dotenv::Error> for ServerError {
    fn from(_: dotenv::Error) -> Self {
        ServerError::InternalError("Unable to load dotenv.".to_owned())
    }
}

impl From<config::ConfigError> for ServerError {
    fn from(e: config::ConfigError) -> Self {
        ServerError::InternalError(format!("Unable to parse: {}", e))
    }
}
