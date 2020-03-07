use actix_web::error;
use actix_web::http::StatusCode;
use std::fmt::Formatter;
use std::{fmt, io, ffi};

#[derive(Debug)]
pub enum ServerError {
    DBError(String),
    IOError(io::Error),
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
    fn from(e: io::Error) -> Self {
        ServerError::IOError(e)
    }
}

impl From<r2d2::Error> for ServerError {
    fn from(e: r2d2::Error) -> Self {
        ServerError::DBError(e.to_string())
    }
}

impl From<rusqlite::Error> for ServerError {
    fn from(e: rusqlite::Error) -> Self {
        ServerError::DBError(e.to_string())
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

impl From<ffi::OsString> for ServerError {
    fn from(e: ffi::OsString) -> Self {
        // It's unlikely to happen.
        ServerError::InternalError(format!("{:?}", e))
    }
}
