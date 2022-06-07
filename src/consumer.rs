use std::borrow::Borrow;
use std::io::{ErrorKind, Read};
use std::rc::Rc;
use std::sync::Arc;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message};
use crate::config::Topic;
use serde_json;
use anyhow::Error;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use crate::entity::kedaface::*;
use crate::CONTEXT;
use crate::db::mapper::{KdFaceMapper, AiasKdFaceMapper, FaceMapper, KdStaticFaceMapper, KpmpDataIngestionStaticFaceInputMapper, KpmpDataIngestionStaticPersonInputMapper, StaticFaceMapper};
use futures::executor::block_on;
use kafka::error::KafkaCode::OffsetOutOfRange;
use kafka::producer::{Producer, Record};
use crate::entity::aias_kdface::ImageList;
use crate::entity::face::Face;
use crate::entity::kpmp_face::KpmpPersonFace;
use crate::entity::static_face::{StaticFace, StaticFaceObj};

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
                let msg = String::from_utf8(Vec::from(m.clone().value)).unwrap();
                let err = do_message(topic_alias.clone(), msg, ms.partition(), m.offset).await;
                if let Err(er) = err {
                    println!("{}", er);
                }
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

async fn do_message(topic_name: String, message: String, partition: i32, offset: i64) -> Result<(), Error>{
    match topic_name.as_str() { 
        "kpmp-analysis-kdstaticface" => {

            let kedaface: KedaFace = serde_json::from_str(message.as_str())?;
            println!("消费到二次静态人脸{:?}", kedaface);
            for obj in &kedaface.KedaFaceListObject.KedaFaceObject {
                let mapper = KdStaticFaceMapper::from(obj.clone());
                let result = &CONTEXT.inner.read().unwrap().rbatis.save(&mapper, &[]).await;
                if let Err(e) = result {
                    panic!("{}", e);
                }
            }
        }
        "kpmp-analysis-kdface" => {

            let kedaface: KedaFace = serde_json::from_str(message.as_str())?;
            println!("消费到二次人脸{:?}", kedaface);
            for obj in &kedaface.KedaFaceListObject.KedaFaceObject {
                let mut mapper = KdFaceMapper::from(obj.clone());
                mapper.DataOffset = offset;
                mapper.DataPartition = partition;
                let result = &CONTEXT.inner.read().unwrap().rbatis.save(&mapper, &[]).await;
                if let Err(e) = result {
                    panic!("{}", e);
                }
            }
        }
        "kpmp-analysis-face" => {

            let face: Face = serde_json::from_str(message.as_str())?;
            println!("消费到一次动态人脸{:?}", face);
            for obj in &face.FaceList.FaceObject {
                let mapper = FaceMapper::from(obj.clone());
                let result = &CONTEXT.inner.read().unwrap().rbatis.save(&mapper, &[]).await;
                if let Err(e) = result {
                    panic!("{}", e);
                }
            }
        }
        "kpmp-analysis-staticface" => {

            let face: StaticFace = serde_json::from_str(message.as_str())?;
            println!("消费到一次静态人脸{:?}", face);
            for obj in &face.FaceList.FaceObject {
                let mapper = StaticFaceMapper::from(obj.clone());
                let result = &CONTEXT.inner.read().unwrap().rbatis.save(&mapper, &[]).await;
                if let Err(e) = result {
                    panic!("{}", e);
                }
            }
        }
        "kpmp-data-ingestion-staticperson-input" => {

            let face: KpmpPersonFace = serde_json::from_str(message.as_str())?;
            println!("消费到接入人像中台录入静态数据{:?}", face);
            if NaiveDateTime::parse_from_str(face.faces.get(0).unwrap().ShotTime.as_str(), "%Y%m%d%H%M%S").unwrap().timestamp() >= NaiveDateTime::parse_from_str("2022-05-31 13:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp()
            && NaiveDateTime::parse_from_str(face.faces.get(0).unwrap().ShotTime.as_str(), "%Y%m%d%H%M%S").unwrap().timestamp() < NaiveDateTime::parse_from_str("2022-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp(){
                Producer::from_hosts(vec!["10.165.76.45:19092".to_owned()]).create().unwrap().send(
                    &Record::from_value("kpmp-data-ingestion-staticperson-input-debug", message.as_bytes())
                );
            }

            /*for obj in &face.faces {
                let mapper = KpmpDataIngestionStaticPersonInputMapper::from(obj.clone());
                let result = &CONTEXT.inner.read().unwrap().rbatis.save(&mapper, &[]).await;
                if let Err(e) = result {
                    panic!("{}", e);
                }
            }*/
        }
        "kpmp-data-ingestion-staticface-input" => {

            let face: StaticFaceObj = serde_json::from_str(message.as_str())?;
            println!("消费到接入人像中台录入静态数据{:?}", face);
            let mapper = KpmpDataIngestionStaticFaceInputMapper::from(face.clone());
            let result = &CONTEXT.inner.read().unwrap().rbatis.save(&mapper, &[]).await;
            if let Err(e) = result {
                panic!("{}", e);
            }
        }
        "aias-kdface" => {

            let image_list: ImageList = serde_json::from_str(message.as_str())?;
            println!("消费到一次动态人脸{:?}", image_list);
            for obj in &image_list.ImageListObject.Image {
                for kdface in &obj.KedaFaceList.KedaFaceObject {
                    let mut mapper = AiasKdFaceMapper::from(kdface.clone());
                    if mapper.ImageID != "" {
                        mapper.DataPartition = partition;
                        mapper.DataOffset = offset;
                        let result = &CONTEXT.inner.read().unwrap().rbatis.save(&mapper, &[]).await;
                        if let Err(e) = result {
                            panic!("{}", e);
                        }
                    }
                }
            }
        }
        _ => panic!("topic do logic error!")
    }
    Ok(())
}