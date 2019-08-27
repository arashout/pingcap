use std::collections::HashMap;

use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::Write;

use failure::{err_msg, Error};
use ron::de::from_reader;
use ron::ser::to_string;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

pub const LOG_FILE: &str = "log.txt";

#[derive(Serialize, Deserialize, Debug)]
enum LogCommand {
    Set { key: String, value: String },
    Rm { key: String },
    Get { key: String },
}
type LogCommands = Vec<LogCommand>;

pub struct KvStore {
    store: HashMap<String, String>,
    log_commands: LogCommands,
    log_path: std::path::PathBuf,
}

impl KvStore {
    pub fn open(log_path: &std::path::Path) -> Result<Self> {
        // let path_buf = log_path.join(LOG_FILE);
        // let log_path = path_buf.as_path();

        let mut kv_store = KvStore::new(log_path);
        if !log_path.exists() {
            let mut file = File::create(log_path)?;
            file.write_all(b"")?;
        }

        let file = File::open(log_path)?;
        let buf_reader = BufReader::new(file);
        let log_commands: LogCommands = from_reader(buf_reader)?;

        for lc in log_commands.iter() {
            kv_store.process_command(lc);
        }
        kv_store.log_commands = log_commands;

        Ok(kv_store)
    }
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.push_command(LogCommand::Get { key }))
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.push_command(LogCommand::Rm { key: key });
        Ok(())
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.push_command(LogCommand::Set {
            key: key.clone(),
            value: value.clone(),
        });
        Ok(())
    }
    fn new(path: &std::path::Path) -> KvStore {
        KvStore {
            store: HashMap::new(),
            log_commands: Vec::new(),
            log_path: path.to_owned(),
        }
    }
    fn push_command(&mut self, log_command: LogCommand) -> Option<String> {
        match log_command {
            LogCommand::Set { key, value } => {
                self.log_commands.push(LogCommand::Set {
                    key: key.clone(),
                    value: value.clone(),
                });
                self.store.insert(key.to_owned(), value.to_owned());
                Some(value)
            }
            LogCommand::Rm { key } => {
                self.log_commands.push(LogCommand::Rm { key: key.clone() });
                self.store.remove(&key.to_owned());
                Some(key)
            }
            LogCommand::Get { key } => self.store.get(&key).map(|v| v.to_owned()),
        }
    }
    fn process_command(&mut self, log_command: &LogCommand) {
        match log_command {
            LogCommand::Set { key, value } => {
                self.store.insert(key.to_owned(), value.to_owned());
            }
            LogCommand::Rm { key } => {
                self.store.remove(&key.to_owned());
            }
            LogCommand::Get { key } => {
                panic!("Should not have get command in log!");
            }
        }
    }
}

impl Drop for KvStore {
    fn drop(&mut self) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.log_path)
            .expect(&format!("Could not read log file in drop {:?}", &self.log_path));

        let content = to_string(&self.log_commands).expect("Failed to serialize command in drop");

        if let Err(e) = write!(file, "{}", content) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
