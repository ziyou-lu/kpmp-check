use serde::Deserialize;

#[derive(Deserialize)]
pub struct KedaFace {
    pub KedaFaceListObject: KedaFaceListObject
}

#[derive(Deserialize)]
pub struct KedaFaceListObject {
    pub KedaFaceObject: Vec<KdStaticFace>,
}

#[derive(Deserialize, Clone)]
pub struct KdStaticFace {
    pub FaceID: String,
    pub ShotTime: String,
}