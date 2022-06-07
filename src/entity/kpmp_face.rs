use serde::Deserialize;
use crate::entity::static_face::{StaticFace, StaticFaceObj};

#[derive(Deserialize, Debug)]
pub struct KpmpPersonFace {
    pub faces: Vec<StaticFaceObj>,
}