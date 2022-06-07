use crate::entity::image::KedaSubImageList;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ImageList {
    pub ImageListObject: ImageListObject,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageListObject {
    pub Image: Vec<ImageObject>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageObject {
    pub KedaFaceList: KedaFaceList,
}

#[derive(Deserialize, Clone, Debug)]
pub struct KedaFaceList {
    pub KedaFaceObject: Vec<KdFace>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct KdFace {
    pub FaceID: String,
    pub ShotTime: String,
    pub TabID: String,
    pub KedaSubImageList: KedaSubImageList,
}