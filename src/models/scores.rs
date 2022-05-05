use juniper::{futures::StreamExt, GraphQLObject};
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

use bson::{doc, Bson};

use super::score::Score;
use crate::DATABASE;

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct Scores {
    pub bests: Vec<Score>,
    pub recents: Vec<Score>,
}

impl Scores {
    pub async fn from_user_id(user_id: i32, mode: i32) -> Self {
        let scores = DATABASE.get().unwrap().collection("scores");

        let mut bests: Vec<Score> = Vec::new();
        let mut recents: Vec<Score> = Vec::new();

        let find_options = FindOptions::builder().sort(doc! {"pp": -1f64}).build();
        let mut bests_bson = scores
            .find(
                doc! {"user_id": user_id, "status": 2i32, "mode": mode},
                find_options,
            )
            .await
            .unwrap();

        loop {
            let db_bests_res = bests_bson.next().await;

            let db_best = match db_bests_res {
                Some(best) => best,
                _ => break,
            };

            match db_best {
                Ok(best) => {
                    let best_obj: Score = bson::from_bson(Bson::Document(best)).unwrap();
                    bests.push(best_obj);
                }
                _ => break,
            };
        }

        let find_options = FindOptions::builder().sort(doc! {"id": -1i32}).build();
        let mut recents_bson = scores
            .find(doc! {"user_id": user_id, "mode": mode}, find_options)
            .await
            .unwrap();

        loop {
            let db_recents_res = recents_bson.next().await;

            let db_recent = match db_recents_res {
                Some(recent) => recent,
                _ => break,
            };

            match db_recent {
                Ok(recent) => {
                    let recent_obj: Score = bson::from_bson(Bson::Document(recent)).unwrap();
                    recents.push(recent_obj);
                }
                _ => break,
            };
        }

        Self { bests, recents }
    }
}
