use juniper::futures::StreamExt;
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

    pub async fn search(query: String) -> Vec<Self> {
        let mut user_vec: Vec<Self> = Vec::new();

        let users = DATABASE.get().unwrap().collection("users");
        let mut db_bson = users
            .find(doc! {"$or": [{"name": {"$regex": query.clone(), "$options": "i"}}, {"safe_name": {"$regex": query.clone(), "$options": "i"}}]}, None)
            .await
            .unwrap();

        loop {
            let db_user_res = db_bson.next().await;

            let db_user = match db_user_res {
                Some(user) => user,
                _ => break,
            };

            match db_user {
                Ok(user) => {
                    let user_obj: User = bson::from_bson(Bson::Document(user)).unwrap();
                    user_vec.push(user_obj);
                }
                _ => break,
            }
        }

        user_vec
    }
}
