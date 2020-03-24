use std::borrow::Cow;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Encoding { best_match: String },
    UnknownEncoding,
    Decoding(String),
    CsvError(csv::Error),
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

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Error::CsvError(e)
    }
}
