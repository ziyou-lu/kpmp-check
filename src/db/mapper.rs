use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Utc};
use rbatis::{crud_table, DateTimeNative};
use serde::Deserialize;
use crate::entity::kedaface::KdStaticFace;

#[crud_table(table_name:"kpmp_analysis_kdstaticface")]
pub struct KdStaticFaceMapper {
    pub FaceID: String,
    pub ShotTime: rbatis::DateTimeNative,
    pub FeatureData: String,
}

impl From<KdStaticFace> for KdStaticFaceMapper {

    fn from(face: KdStaticFace) -> Self {
        let fmt = "%Y%m%d%H%M%S";
        let mut kdstaticface = KdStaticFaceMapper {
            FaceID: face.FaceID,
            ShotTime: DateTimeNative::from(DateTime::<Utc>::from_utc(NaiveDateTime::parse_from_str(face.ShotTime.as_str(), fmt).unwrap(), Utc).with_timezone(&Local)),
            FeatureData: "".to_string()
        };
        if face.KedaSubImageList.KedaSubImageInfoObject.is_empty() {
            return kdstaticface;
        }

        if face.KedaSubImageList.KedaSubImageInfoObject.get(0).unwrap().Feature.is_empty() {
            return kdstaticface;
        }
        if let Some(data) = face.KedaSubImageList.KedaSubImageInfoObject.get(0).unwrap().clone().Feature.get(0).unwrap().FeatureData.clone() {
            kdstaticface.FeatureData = data;
        }
        kdstaticface
    }
}