use serde::Deserialize;
use crate::entity::image::KedaSubImageList;

#[derive(Deserialize)]
pub struct Face {
    pub FaceList: FaceListObject
}

#[derive(Deserialize)]
pub struct FaceListObject {
    pub FaceObject: Vec<StaticFace>,
}

#[derive(Deserialize, Clone)]
pub struct StaticFace {
    pub FaceID: String,
    pub ShotTime: String,
    pub TabID: String,
}