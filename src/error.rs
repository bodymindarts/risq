use crate::prelude::{
    sync::{
        mpsc::error::{RecvError, SendError},
        oneshot,
    },
    MailboxError,
};
use prost::{DecodeError, EncodeError};
use std::io;

#[derive(Debug)]
pub enum Error {
    ToSocketError,
    ServerShutdown,
    IoError(io::Error),
    Decode(DecodeError),
    Encode(EncodeError),
    SendOneshotError,
    ReceiveOneshotError,
    MailboxError(MailboxError),
    SendMPSCError,
    ReceiveMPSCError,
    ConnectionClosed,
    DidNotReceiveExpectedResponse,
}

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
impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Self {
        Error::Encode(err)
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
impl From<oneshot::error::RecvError> for Error {
    fn from(_err: oneshot::error::RecvError) -> Self {
        Error::ReceiveOneshotError
    }
}
