use failure::Error;
use serde;
use serde_json;
use std::io;

/**
 * custom error type to indicate different error
 */
#[derive(Fail, Debug)]
pub enum KvError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "Key not found")]
    KeyNotFound,

    #[fail(display = "Invalid command")]
    InvalidCommand,
}

impl From<io::Error> for KvError {
    fn from(e: io::Error) -> KvError {
        KvError::Io(e)
    }
}

impl From<serde_json::Error> for KvError {
    fn from(e: serde_json::Error) -> KvError {
        KvError::Serde(e)
    }
}

pub type Result<T> = std::result::Result<T, KvError>;
