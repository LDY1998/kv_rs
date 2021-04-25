use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct KvStore {
    values: HashMap<String, String>,
    writer: BufWriter<File>,
}

/**
 * ! Command that used to serialize
 */
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
}

impl KvStore {
    pub fn get(&self, key: String) -> Option<String> {
        match self.values.get(&key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key,
            value: value,
        };

        serde_json::to_writer(&mut self.writer, &cmd);
        self.writer.flush();

        // TODO: It then appends the serialized command to a file containing the log
        // TODO: Needs to figure out how the log and index work
        // append()

        Ok(())
    }

    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();

        Ok(KvStore {
            values: HashMap::new(),
            writer: BufWriter::new(File::open(&path)?),
        })
    }
}
