use kv::error::KvError;
use kv::kvs::KvStore;
use std::env::current_dir;
use std::process::exit;
use structopt::StructOpt;

fn main() {
    let path = current_dir().expect("fail to get current directory");
    let mut store = KvStore::open(&path).expect("fail to create and load the dir");
    match KvCli::from_args() {
        KvCli::Get { key } => {
            match store.get(key) {
                Ok(Some(value)) => println!("{}", value),
                Ok(None) => println!("Key not found"),
                Err(e) => {
                    eprintln!("{:?}", e);
                    exit(1)
                }
            }
            exit(0);
        }
        KvCli::Set { key, value } => {
            store.set(key, value).expect("set fail");
            exit(0);
        }
        KvCli::Remove { key } => match store.remove(key) {
            Ok(_) => exit(0),
            Err(KvError::KeyNotFound) => {
                println!("Key not found");
                exit(1)
            }
            _ => exit(1),
        },
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
enum KvCli {
    #[structopt(name = "get")]
    Get { key: String },

    #[structopt(name = "set")]
    Set { key: String, value: String },

    #[structopt(name = "rm")]
    Remove { key: String },
}
