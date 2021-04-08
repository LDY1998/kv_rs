extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use std::process::exit;

fn main() {

    let matches = App::new("kv")
            .version("0.1.0")
            .author("LDY1998")
            .about("A simple key value store using rust")
            .setting(AppSettings::DisableHelpSubcommand)
            .subcommand(
                SubCommand::with_name("get")
                    .about("Get the value of a given key string")
                    .arg(Arg::with_name("Key").help("A string key").required(true))
            )
            .get_matches();

}
