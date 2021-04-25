#[macro_use]
extern crate structopt;
extern crate clap;
mod kvs;
pub use kvs::KvStore;

extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate serde;
mod error;
pub use error::Result;