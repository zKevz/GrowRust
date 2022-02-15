use std::{fmt, io, num::ParseIntError, result, str::Utf8Error};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    StrError(Utf8Error),
    NameError,
    JsonError(serde_json::Error),
    ParseError(ParseIntError),
    NotInWorld,
    IndexError,
    Disconnected,
    ItemNotFound,
    WrongPassword,
    TooManyItemError,
    NameAlreadyExists,
    ItemCountNegative,
    InventoryFullError,
    InvalidPacketError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::StrError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonError(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.to_string();

        f.write_fmt(format_args!("Error {{ message: {} }}", message))
    }
}
