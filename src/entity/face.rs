use serde::Deserialize;
use crate::entity::image::KedaSubImageList;

#[derive(Deserialize, Debug)]
pub struct Face {
    pub FaceList: FaceListObject
}

#[derive(Deserialize, Debug)]
pub struct FaceListObject {
    pub FaceObject: Vec<FaceObj>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FaceObj {
    pub FaceID: String,
    pub ShotTime: String,
    pub DeviceID: String
}