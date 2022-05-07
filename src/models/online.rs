use bson::{doc, Bson};
use juniper::{futures::StreamExt, GraphQLObject};
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

use crate::DATABASE;

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct RecentUser {
    pub id: i32,
    pub name: String,
    pub register_time: i32,
}

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct OnlineInfo {
    pub count: i32,
    pub online: i32,
    pub recents: Vec<RecentUser>,
}

impl OnlineInfo {
    pub async fn get_online_info() -> Self {
        let users = DATABASE.get().unwrap().collection("users");

        let count = users.count_documents(doc! {}, None).await.unwrap();

        let find_options = FindOptions::builder()
            .sort(doc! {"id": -1i32})
            .limit(5)
            .build();

        let mut recents = users.find(doc! {}, find_options).await.unwrap();
        let mut recent_vec: Vec<RecentUser> = Vec::new();

        loop {
            let db_recents_res = recents.next().await;

            let db_recent = match db_recents_res {
                Some(recent) => recent,
                _ => break,
            };

            match db_recent {
                Ok(recent) => {
                    let recent_obj: RecentUser = bson::from_bson(Bson::Document(recent)).unwrap();
                    recent_vec.push(recent_obj);
                }
                _ => break,
            }
        }

        let online = 0i32; // temporary

        Self {
            count: count.try_into().unwrap(),
            online,
            recents: recent_vec,
        }
    }
}
