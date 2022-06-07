use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, Utc};
use futures::SinkExt;
use rbatis::{crud_table, DateTimeNative};
use serde::Deserialize;
use toml::value::Offset;
use crate::entity::aias_kdface::KdFace;
use crate::entity::face::{Face, FaceObj};
use crate::entity::static_face::{StaticFace, StaticFaceObj};
use crate::entity::kedaface::KdStaticFace;
use crate::entity::kpmp_face::KpmpPersonFace;

#[crud_table(table_name:"kpmp_analysis_kdstaticface")]
pub struct KdStaticFaceMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub TabID: String,
    pub FeatureData: String,
}

impl From<KdStaticFace> for KdStaticFaceMapper {

    fn from(face: KdStaticFace) -> Self {
        let fmt = "%Y%m%d%H%M%S";
        let mut kdstaticface = KdStaticFaceMapper {
            FaceID: face.FaceID,
            ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), FixedOffset::west(8 * 3600))),
            TabID: face.TabID,
            FeatureData: "".to_string()
        };
        if face.KedaSubImageList.KedaSubImageInfoObject.is_empty() {
            return kdstaticface;
        }

        if face.KedaSubImageList.KedaSubImageInfoObject.get(0).unwrap().Feature.as_ref().unwrap().is_empty() {
            return kdstaticface;
        }
        if let Some(data) = face.KedaSubImageList.KedaSubImageInfoObject.get(0).unwrap().clone().Feature.as_ref().unwrap().get(0).unwrap().FeatureData.clone() {
            kdstaticface.FeatureData = data;
        }
        kdstaticface
    }
}

#[crud_table(table_name:"kpmp_analysis_kdface")]
pub struct KdFaceMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub DeviceID: String,
    pub FeatureData: String,
    pub DataPartition: i32,
    pub DataOffset: i64,
}

impl From<KdStaticFace> for KdFaceMapper {

    fn from(face: KdStaticFace) -> Self {
        let fmt = "%Y%m%d%H%M%S";
        let mut kdface = KdFaceMapper {
            FaceID: face.FaceID,
            ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), FixedOffset::west(8 * 3600))),
            DeviceID: face.DeviceID,
            FeatureData: "".to_string(),
            DataPartition: 0,
            DataOffset: 0,
        };
        if face.KedaSubImageList.KedaSubImageInfoObject.is_empty() {
            return kdface;
        }

        if face.KedaSubImageList.KedaSubImageInfoObject.get(0).unwrap().Feature.as_ref().unwrap().is_empty() {
            return kdface;
        }
        if let Some(data) = face.KedaSubImageList.KedaSubImageInfoObject.get(0).unwrap().clone().Feature.as_ref().unwrap().get(0).unwrap().FeatureData.clone() {
            kdface.FeatureData = data;
        }
        kdface
    }
}

#[crud_table(table_name:"kpmp_analysis_staticface")]
pub struct StaticFaceMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub TabID: String,
}

impl From<StaticFaceObj> for StaticFaceMapper {

    fn from(face: StaticFaceObj) -> Self {
        let fmt = "%Y%m%d%H%M%S";
        StaticFaceMapper {
            FaceID: face.FaceID,
            ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), FixedOffset::west(8 * 3600))),
            TabID: face.TabID,
        }
    }
}

#[crud_table(table_name:"kpmp_analysis_face")]
pub struct FaceMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub DeviceID: String,
}

impl From<FaceObj> for FaceMapper {

    fn from(face: FaceObj) -> Self {
        let fmt = "%Y%m%d%H%M%S";
        FaceMapper {
            FaceID: face.FaceID,
            ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), FixedOffset::west(8 * 3600))),
            DeviceID: face.DeviceID,
        }
    }
}

#[crud_table(table_name:"kpmp_data_ingestion_staticperson_input")]
pub struct KpmpDataIngestionStaticPersonInputMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub TabID: String,
}

impl From<StaticFaceObj> for KpmpDataIngestionStaticPersonInputMapper {

    fn from(face: StaticFaceObj) -> Self {
        let fmt = "%Y%m%d%H%M%S";
        let time = DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(),FixedOffset::west(8 * 3600)));
        KpmpDataIngestionStaticPersonInputMapper {
            FaceID: face.FaceID,
            ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(),FixedOffset::west(8 * 3600))),
            TabID: face.TabID,
        }
    }
}

#[crud_table(table_name:"kpmp_data_ingestion_staticface_input")]
pub struct KpmpDataIngestionStaticFaceInputMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub TabID: String,
}

impl From<StaticFaceObj> for KpmpDataIngestionStaticFaceInputMapper {

    fn from(face: StaticFaceObj) -> Self {
        let fmt = "%Y%m%d%H%M%S";
        KpmpDataIngestionStaticFaceInputMapper {
            FaceID: face.FaceID,
            ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(),FixedOffset::west(8 * 3600))),
            TabID: face.TabID,
        }
    }
}

#[crud_table(table_name:"aias_kdface")]
pub struct AiasKdFaceMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub ImageID: String,
    pub Url: String,
    pub FeatureInfo: String,
    pub DataPartition: i32,
    pub DataOffset: i64,
}

impl From<KdFace> for AiasKdFaceMapper {

        fn from(face: KdFace) -> Self {
            let fmt = "%Y%m%d%H%M%S";
            for sub_image in face.KedaSubImageList.KedaSubImageInfoObject {
                if sub_image.Type != "11" {
                    continue;
                }
                if !sub_image.Feature.is_none() && !sub_image.Feature.as_ref().unwrap().is_empty() {
                    return AiasKdFaceMapper {
                        FaceID: face.FaceID,
                        ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), FixedOffset::west(8 * 3600))),
                        ImageID: sub_image.ImageID,
                        Url: sub_image.StoragePath.clone().unwrap_or("".to_string()),
                        FeatureInfo: sub_image.Feature.unwrap().get(0).unwrap().FeatureData.clone().unwrap_or("".to_string()),
                        DataPartition: 0,
                        DataOffset: 0,
                    }
                } else {
                    return AiasKdFaceMapper {
                        FaceID: face.FaceID,
                        ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), FixedOffset::west(8 * 3600))),
                        ImageID: sub_image.ImageID,
                        Url: sub_image.StoragePath.clone().unwrap_or("".to_string()),
                        FeatureInfo: "".to_string(),
                        DataPartition: 0,
                        DataOffset: 0,
                    }
                }
            }

            AiasKdFaceMapper {
                FaceID: face.FaceID,
                ShotTime: DateTimeNative::from(DateTime::<>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), FixedOffset::west(8 * 3600))),
                ImageID: "".to_string(),
                Url: "".to_string(),
                FeatureInfo: "".to_string(),
                DataPartition: 0,
                DataOffset: 0,
            }
        }
}