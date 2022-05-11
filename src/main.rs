
mod config;
mod consumer;
mod entity;
mod db;
use config::{Config};
use rbatis::rbatis::Rbatis;
use tokio::io::AsyncReadExt;
use std::sync::{RwLock, Arc};
use std::{task, thread};
use std::thread::Thread;
use std::time::Duration;
use futures::executor::block_on;
use futures::future::join_all;
use crate::entity::image::Feature;

#[macro_use]
extern crate lazy_static;

pub struct  Context {
    inner: Arc<RwLock<ContextInner>>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

pub struct ContextInner {
    pub config: Config,
    pub rbatis: Rbatis,
    pub rbatis_inited: bool,
}

unsafe impl Send for ContextInner {}
unsafe impl Sync for ContextInner {}


impl Context {
    pub fn new() -> Self {
        let init_config = Config::read_config("config.toml");
        Context {
            inner : Arc::new ( RwLock::new ( ContextInner {
                config: init_config.clone(),
                rbatis: crate::db::init_rbatis(&init_config),
                rbatis_inited: false,
            })),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut context_write = CONTEXT.inner.write().unwrap();
    block_on(context_write.rbatis.link(&context_write.config.db.url)).expect("[abs_admin] rbatis link database fail!");
    drop(context_write);
    let mut futures = vec![];
    for topic in CONTEXT.inner.read().unwrap().config.topics.clone() {
        let topic_c = topic.clone();
        let hosts_c = CONTEXT.inner.read().unwrap().config.kafka.hosts.clone();
        futures.push(consumer::start_consumer(hosts_c, topic_c));
        println!("topic consumer inited: {:?}", topic.name.clone());
    }

    join_all(futures).await;
}

lazy_static! {
    pub static ref CONTEXT: Context = Context::new();
}

