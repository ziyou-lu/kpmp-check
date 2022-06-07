use serde::Deserialize;
use crate::entity::image::KedaSubImageList;

#[derive(Deserialize, Debug)]
pub struct KedaFace {
    pub KedaFaceListObject: KedaFaceListObject
}

#[derive(Deserialize, Debug)]
pub struct KedaFaceListObject {
    pub KedaFaceObject: Vec<KdStaticFace>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct KdStaticFace {
    pub FaceID: String,
    pub ShotTime: String,
    pub TabID: String,
    pub KedaSubImageList: KedaSubImageList,
    pub DeviceID: String,
}