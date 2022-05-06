use bson::{doc, Bson};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::DATABASE;

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct Beatmap {
    pub id: i32,
    pub ar: f64,
    pub artist: String,
    pub bpm: f64,
    pub creator: String,
    pub cs: f64,
    pub diff: f64,
    pub filename: String,
    pub frozen: bool,
    pub hp: f64,
    pub last_update: String,
    pub max_combo: i32,
    pub md5: String,
    pub mode: i32,
    pub od: f64,
    pub passes: i32,
    pub plays: i32,
    pub set_id: i32,
    pub status: i32,
    pub title: String,
    pub total_length: i32,
    pub version: String,
}

impl Beatmap {
    pub async fn from_md5(md5: String) -> Option<Self> {
        let maps = DATABASE.get().unwrap().collection("maps");

        let db_bson = maps.find_one(doc! {"md5": md5}, None).await.unwrap();
        let bson_document = match db_bson {
            Some(doc) => doc,
            None => return None,
        };

        Some(bson::from_bson(Bson::Document(bson_document)).unwrap())
    }
}
