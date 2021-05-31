use std::fmt;

use crate::bindings as C;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Msg(&'static str),
    Status(C::TfLiteStatus::Type),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn from_msg(msg: &'static str) -> Self {
        Self {
            kind: ErrorKind::Msg(msg),
        }
    }

    pub(crate) fn from_status(status: C::TfLiteStatus::Type) -> Self {
        Self {
            kind: ErrorKind::Status(status),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f) // FIXME
    }
}

impl std::error::Error for Error {}
