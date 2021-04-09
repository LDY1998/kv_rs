extern crate clap;

#[macro_use]
extern crate structopt;

use clap::{Arg, App, SubCommand, AppSettings};
use std::process::exit;
use structopt::StructOpt;

fn main() {

    match KvCli::from_args() {
        KvCli::Get {key} => {
            eprintln!("unimplemented");
            exit(1);
        },
        KvCli::Set {key, value} => {
            eprintln!("unimplemented");
            exit(1);
        },
        KvCli::Remove {key} => {
            eprintln!("unimplemented");
            exit(1);
        }
    }

}

// #[derive(StructOpt, Debug)]
// #[structopt(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
// struct Opt {
//     #[structopt(subcommand)]
//     subcommand: KvCli
// }

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
enum KvCli { 
    #[structopt(name = "get")]
    Get {
        key: String,
    },

    #[structopt(name="set")]
    Set {
        key: String,
        value: String,
    },

    #[structopt(name="rm")]
    Remove {
        key: String
    }

}