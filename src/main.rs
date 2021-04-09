extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use std::process::exit;

fn main() {

    let matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .setting(AppSettings::DisableHelpSubcommand)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::VersionlessSubcommands)
            .subcommand(
                SubCommand::with_name("get")
                    .about("Get the value of a given key string")
                    .arg(Arg::with_name("Key").help("A string key").required(true))
            )
            .subcommand(
                SubCommand::with_name("set")
                    .about("Set the value of given key")
                    .arg(Arg::with_name("Key").help("A string key").required(true))
                    .arg(Arg::with_name("Value").help("A string value").required(true))
            )
            .subcommand(
                SubCommand::with_name("rm")
                    .about("Remove the value with given key")
                    .arg(Arg::with_name("Key").help("A string key").required(true))
            )
            .get_matches();

            match matches.subcommand() {
                ("get", Some(key)) => {
                   eprintln!("unimplemented");
                   exit(1); 
                },
                ("set", Some(kv)) => {
                   eprintln!("unimplemented");
                   exit(1); 
                },
                ("rm", Some(key)) => {
                    eprintln!("unimplemented");
                    exit(1);
                },
                (_) => {
                    eprintln!("unimplemented");
                    exit(1);
                },
            }

}
