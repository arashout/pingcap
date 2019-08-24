use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Write;
use std::io::BufReader;

use failure::{err_msg, Error};
use ron::de::from_reader;
use ron::ser::to_string;
use serde::{Deserialize, Serialize};

pub type CustomResult<T> = std::result::Result<T, Error>;

const LOG_FILE: &str = "log.txt";

#[derive(Serialize, Deserialize, Debug)]
enum LogCommand {
    Set { key: String, value: String },
    Rm { key: String },
    Get { key: String },
}
type LogCommands = Vec<LogCommand>;

pub struct KvStore {
    store: HashMap<String, String>,
}

// TODO: Implement Drop?
impl KvStore {
    pub fn open() -> CustomResult<Self> {
        let mut kv_store = KvStore::new();
        if let Ok(f) = File::open(LOG_FILE) {
            // Already have a log file
            let reader = BufReader::new(f);
            let log_commands: Vec<LogCommand> = from_reader(reader)?;
            for lc in log_commands {
                kv_store.process_command(lc);
            }
        } else {
            // Create a brand new log file
            let log_commands: LogCommands = vec![];
            if let Ok(mut f) = File::create(LOG_FILE) {
                let content = to_string(&log_commands)?;
                f.write_all(content.as_bytes())?;
            }
        }
        Ok(kv_store)
    }
    pub fn get(&self, key: String) -> CustomResult<Option<String>> {
        self.store
            .get(&key)
            .map(|v| Some(v.to_owned()))
            .ok_or_else(|| err_msg("Could not open log"))
    }
    pub fn remove(&mut self, key: String) -> CustomResult<()> {
        self.store
            .remove(&key)
            .map(|_v| ())
            .ok_or_else(|| err_msg("Could not open log"))
    }
    pub fn set(&mut self, key: String, value: String) -> CustomResult<()> {
        self.store
            .insert(key, value)
            .map(|_v| ())
            .ok_or_else(|| err_msg("Could not open log"))
    }
    fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }
    fn process_command(&mut self, log_command: LogCommand) {
        match log_command {
            LogCommand::Set { key, value } => {
                self.store.insert(key, value);
            }
            LogCommand::Rm { key } => {
                self.store.remove(&key);
            }
            LogCommand::Get { key } => {}
        }
    }
}
