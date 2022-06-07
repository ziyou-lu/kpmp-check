use serde::Deserialize;
use crate::entity::image::KedaSubImageList;

#[derive(Deserialize, Debug)]
pub struct StaticFace {
    pub FaceList: StaticFaceListObject
}

#[derive(Deserialize, Debug)]
pub struct StaticFaceListObject {
    pub FaceObject: Vec<StaticFaceObj>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct StaticFaceObj {
    pub FaceID: String,
    pub ShotTime: String,
    pub TabID: String,
}