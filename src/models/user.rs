use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::stats::Stats;

use crate::DATABASE;
use bson::{doc, Bson};

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub safe_name: String,
    pub register_time: i32,
    pub latest_activity: i32,
    pub privileges: i32,
    pub silence_end: i32,
    pub friends: Vec<i32>,
    pub country: String,
    pub blocked: Vec<i32>,

    pub stats: Option<Vec<Stats>>,
}

impl User {
    pub async fn from_user_id(user_id: i32) -> Option<Self> {
        let users = DATABASE.get().unwrap().collection("users");

        let db_bson = users.find_one(doc! {"id": user_id}, None).await.unwrap();
        let bson_document = match db_bson {
            Some(doc) => doc,
            None => return None,
        };

        Some(bson::from_bson(Bson::Document(bson_document)).unwrap())
    }

    pub async fn from_user_name(name: String) -> Option<Self> {
        let users = DATABASE.get().unwrap().collection("users");

        let db_bson = users.find_one(doc! {"name": name}, None).await.unwrap();
        let bson_document = match db_bson {
            Some(doc) => doc,
            None => return None,
        };

        Some(bson::from_bson(Bson::Document(bson_document)).unwrap())
    }
}
