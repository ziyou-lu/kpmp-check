use serde::{Deserialize};
use std::{fs::File, io::Read};
#[derive(Deserialize, Clone)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub topics: Vec<Topic>,
    pub db: DBConfig,
}

#[derive(Deserialize, Clone)]
pub struct KafkaConfig {
    pub hosts: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct Topic {
    pub name: String,
    pub alias: String,
    pub fallback_offset: String,
    pub group: String,
}

#[derive(Deserialize, Clone)]
pub struct DBConfig {
    pub url: String,
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
}

impl Config { 
    pub fn read_config(file_path: &str) -> Self {
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file: {}, error: {}", file_path, e),
        };

        let mut file_content = String::new();
        if let Err(e) = file.read_to_string(&mut file_content) {
            panic!("read file context error! file: {} error: {}", file_path, e);
        }

        let init_config = match toml::from_str(&file_content) {
            Ok(c) => c,
            Err(e) => panic!(
                "load init config to cache error, file: {}, error: {}",
                file_path, e
            ),
        };

        init_config
    }
}