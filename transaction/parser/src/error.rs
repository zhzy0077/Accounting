use std::io;
use std::borrow::Cow;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Encoding { best_match: String },
    UnknownEncoding,
    Decoding(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<Cow<'static, str>> for Error {
    fn from(e: Cow<'static, str>) -> Self {
        Error::Decoding(e.to_string())
    }
}

