use actix::MailboxError;
use prost::DecodeError;
use std::{io, result};
use tokio::sync::mpsc::error::{RecvError, SendError};

#[derive(Debug)]
pub enum Error {
    ToSocketError,
    ServerShutdown,
    IoError(io::Error),
    Decode(DecodeError),
    SendOneshotError,
    ReceiveOneshotError,
    MailboxError(MailboxError),
    SendMPSCError,
    ReceiveMPSCError,
    ConnectionClosed,
    DidNotReceiveExpectedResponse,
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
impl From<SendError> for Error {
    fn from(_err: SendError) -> Self {
        Error::SendMPSCError
    }
}
impl From<RecvError> for Error {
    fn from(_err: RecvError) -> Self {
        Error::ReceiveMPSCError
    }
}
impl From<MailboxError> for Error {
    fn from(err: MailboxError) -> Self {
        Error::MailboxError(err)
    }
}
