use failure::Error;
use std::io;
use serde;
use serde_json;

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
    InvalidCommand
}

pub type Result<T> = std::result::Result<T, KvError>;