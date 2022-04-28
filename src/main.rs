
mod config;
mod consumer;
mod entity;
mod db;
use config::{Config};
use rbatis::rbatis::Rbatis;
use tokio::io::AsyncReadExt;
use std::sync::{RwLock, Arc};
use std::thread;
use std::thread::Thread;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

pub struct Context {
    pub config: Config,
    pub rbatis: Rbatis,
    pub rbatis_inited: bool,
}

impl Context {
    pub fn new() -> Self {
        let init_config = Config::read_config("config.toml");
        Context { 
            config: init_config.clone(),
            rbatis: crate::db::init_rbatis(&init_config),
            rbatis_inited: false
        }
    }
}

#[tokio::main]
async fn main() {
    let mut context_write = CONTEXT.write().unwrap();
    context_write.rbatis.link(&context_write.config.db.url).await.expect("[abs_admin] rbatis link database fail!");
    drop(context_write);
    for topic in CONTEXT.read().unwrap().config.topics.clone() {
        consumer::start_consumer(CONTEXT.read().unwrap().config.kafka.hosts.clone(), topic.clone()).await;
    }
}

lazy_static! {
    pub static ref CONTEXT: Arc<RwLock<Context>> = Arc::new(RwLock::new(Context::new()));
}

