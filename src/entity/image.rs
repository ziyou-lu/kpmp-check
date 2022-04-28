use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct KedaSubImageList {
    pub KedaSubImageInfoObject: Vec<KedaSubImageInfoObject>,
}
#[derive(Deserialize, Clone)]
pub struct KedaSubImageInfoObject {
    pub Feature: Vec<Feature>,
}
#[derive(Deserialize, Clone)]
pub struct Feature {
    pub FeatureData: Option<String>,
}