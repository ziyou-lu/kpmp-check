use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message};
use crate::config::Topic;
use serde_json;
use anyhow::Error;
use rbatis::crud::CRUD;
use crate::entity::kedaface::*;
use crate::CONTEXT;
use crate::db::mapper::KdStaticFaceMapper;

pub async fn start_consumer(hosts: Vec<String>, topic: Topic) {
    let mut consumer =
    Consumer::from_hosts(hosts)
       .with_topic(topic.name.clone())
       .with_fallback_offset(parse_fetch_offset(topic.fallback_offset.clone()))
       .with_group(topic.group.as_str().to_owned())
       .with_offset_storage(GroupOffsetStorage::Kafka)
       .with_fetch_max_bytes_per_partition(128 * 1024 * 1024)
       .create()
        .unwrap();
    loop {
        let topic_alias = topic.alias.clone();
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}", String::from_utf8(Vec::from(m.clone().value)).unwrap());
                let _ = do_message(topic_alias.clone(), m).await.unwrap();
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}

fn parse_fetch_offset(fetch_offset: String) -> FetchOffset {
    match fetch_offset.as_str() {
        "earliest" => FetchOffset::Earliest,
        "latest" => FetchOffset::Latest,
        _ => panic!("config fetch_offset error!")
    }
}

async fn do_message(topic_name: String, message: &Message<'_>) -> Result<(), Error>{
    match topic_name.as_str() { 
        "kpmp-analysis-kdstaticface" => {
            let kedaface: KedaFace = serde_json::from_str(String::from_utf8(Vec::from(message.value)).unwrap().as_str())?;
            for obj in &kedaface.KedaFaceListObject.KedaFaceObject {
                let mapper = KdStaticFaceMapper::from(obj.clone());
                let result = CONTEXT.read().unwrap().rbatis.save(&mapper, &[]).await;
                if let Err(e) = result {
                    println!("{}", e);
                }
            }
        }
        _ => panic!("topic do logic error!")
    }
    Ok(())
}