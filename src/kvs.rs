use crate::error::{KvError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub struct KvStore {
    writer: BufWriterWithPos,
    index: BTreeMap<String, CommandPos>,
    readers: HashMap<u64, BufReaderWithPos>,
    curr_gen: u64,
}

struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

struct BufWriterWithPos {
    writer: BufWriter<File>,
    pos: u64,
}

impl BufWriterWithPos {
    fn new(mut inner: BufWriter<File>) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: inner,
            pos: pos,
        })
    }
}

struct BufReaderWithPos {
    reader: BufReader<File>,
    pos: u64,
}

impl BufReaderWithPos {
    fn new(mut inner: BufReader<File>) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: inner,
            pos: pos,
        })
    }
}

impl Write for BufWriterWithPos {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

impl Seek for BufWriterWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}

impl Read for BufReaderWithPos {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl Seek for BufReaderWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

/**
 * ! Command that used to serialize
 */
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl KvStore {
    /**
     * ! impl {kv get key},
     * ! 1. read the entire log
     * ! 2. check if key is in index (index is like a cache of all inserted Command)
     * ! 3. if key is not in the index, report KeyNotFound directly
     * ! 4. get the reader from readers map through gen
     * ! 5. seek the reader to the position of the file, and take only at the length of len
     * ! 6. deserialize with serde::from_reader, return the value
     */
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // TODO: read entire log and build the index

        if let Some(pos) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&pos.gen)
                .expect("cannot find the log reader");
            reader.seek(SeekFrom::Start(pos.pos))?;
            let reader = reader.take(pos.len);
            if let Command::Set { value, .. } = serde_json::from_reader(reader)? {
                Ok(Some(value))
            } else {
                Err(KvError::InvalidCommand)
            }
        } else {
            Err(KvError::KeyNotFound)
        }
    }

    /**
     * ! impl {kv set key value}
     * ! get the file offset of the writer
     * ! serialize the Command structure into the offset of that file
     * ! insert the (key, CommandPos) pair into index
     */
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value: value,
        };

        let pos = self.writer.pos;

        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;

        // ! insert a (key, CommandPos) pair into index as a cache in memory
        // ! the index is implemented with a BTreeMap
        self.index.insert(
            key,
            CommandPos {
                gen: self.curr_gen,
                pos: self.writer.pos,
                len: self.writer.pos - pos,
            },
        );

        Ok(())
    }

    /**
     * ! impl {kv remove key}
     * ! 1. read the log and build index
     * ! 2. if the log with the key presents, serialize the
     */
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let Some(pos) = self.index.get(&key) {
            let cmd = Command::Remove { key: key.clone() };
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            self.index.remove(&key).expect("key not found");
            Ok(())
        } else {
            Err(KvError::KeyNotFound)
        }

        // Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        std::fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let gens = read_gens(&path)?;

        for &gen in &gens {
            let log_p = log_path(&path, gen)?;
            let mut reader = BufReaderWithPos::new(BufReader::new(File::open(&log_p)?))?;
            load(gen, &mut reader, &mut index)?;
            readers.insert(gen, reader);
        }

        let current_gen = gens.last().unwrap_or(&0) + 1;

        let writer = new_log_file(&path, current_gen, &mut readers)?;

        Ok(KvStore {
            writer: writer,
            readers: readers,
            index: index,
            curr_gen: 0,
        })
    }
}

fn new_log_file(
    path: &Path,
    gen: u64,
    readers: &mut HashMap<u64, BufReaderWithPos>,
) -> Result<BufWriterWithPos> {
    let path = log_path(&path, gen)?;
    let writer = BufWriterWithPos::new(BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    ));
    readers.insert(gen, BufReaderWithPos::new(BufReader::new(File::open(&path)?))?);
    writer
}

fn log_path(path: &Path, gen: u64) -> Result<PathBuf> {
    Ok(path.join(format!("{}.log", gen)))
}

fn read_gens(path: &Path) -> Result<Vec<u64>> {
    let mut gens: Vec<u64> = std::fs::read_dir(path)?
        .flat_map(|res| match res {
            Ok(entry) => Ok(entry.path()),
            _ => Err(KvError::Io),
        })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();

    gens.sort_unstable();

    Ok(gens)
}

/**
 * ! load the whole log file, deserialize and insert into index
 */
fn load(
    gen: u64,
    reader: &mut BufReaderWithPos,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<()> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;

    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();

    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } | Command::Remove { key, .. } => {
                index.insert(
                    key,
                    CommandPos {
                        gen: gen,
                        pos: pos,
                        len: new_pos - pos,
                    },
                );
            }
        }
        pos = new_pos;
    }

    Ok(())
}
