use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct KedaSubImageList {
    pub KedaSubImageInfoObject: Vec<KedaSubImageInfoObject>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct KedaSubImageInfoObject {
    pub ImageID: String,
    pub StoragePath: Option<String>,
    pub Type: String,
    pub Feature: Option<Vec<Feature>>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Feature {
    pub FeatureData: Option<String>,
}