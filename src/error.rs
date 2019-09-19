use prost::DecodeError;
use std::{io, result};

#[derive(Debug)]
pub enum Error {
    ToSocketError,
    IoError(io::Error),
    Decode(DecodeError),
}

#[derive(Debug)]
pub enum ReceiveErrorKind {
    EOF,
    Decode,
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}
impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Error::Decode(err)
    }
}
