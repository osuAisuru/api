use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct Score {
    pub id: i32,
    pub acc: f64,
    pub client_checksum: String,
    pub client_flags: i32,
    pub grade: i32,
    pub map_md5: String,
    pub max_combo: i32,
    pub mode: i32,
    pub mods: i32,
    pub n100: i32,
    pub n300: i32,
    pub n50: i32,
    pub ngeki: i32,
    pub nkatu: i32,
    pub nmiss: i32,
    pub passed: bool,
    pub perfect: bool,
    pub pp: f64,
    pub replay_views: i32,
    pub score: i32,
    pub status: i32,
    pub time: String,
    pub time_elapsed: i32,
    pub user_id: i32,
}
