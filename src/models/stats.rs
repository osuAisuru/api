use juniper::GraphQLObject;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::{DATABASE, REDIS};
use bson::{doc, Bson};
use juniper::futures::StreamExt;

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct Stats {
    pub accuracy: f64,
    pub max_combo: i32,
    pub playcount: i32,
    pub playtime: i32,
    pub pp: i32,
    pub ranked_score: i32,
    pub total_hits: i32,
    pub total_score: i32,
    pub mode: i32,

    pub global_rank: Option<i32>,
    pub country_rank: Option<i32>,
}

impl Stats {
    pub async fn from_user_id(user_id: i32, user_country: String) -> Vec<Self> {
        let stats = DATABASE.get().unwrap().collection("ustats");
        let mut redis_conn = REDIS.get().unwrap().get_async_connection().await.unwrap();

        let mut stats_vec: Vec<Stats> = Vec::new();
        let mut stats_bson = stats.find(doc! {"user_id": user_id}, None).await.unwrap();

        loop {
            let db_stat_res = stats_bson.next().await;

            let db_stat = match db_stat_res {
                Some(db) => db,
                _ => break,
            };

            match db_stat {
                Ok(stat) => {
                    let mut stat_obj: Stats = bson::from_bson(Bson::Document(stat)).unwrap();

                    let leaderboard_str = format!("aisuru:leaderboard:{}", stat_obj.mode);
                    let country_leaderboard_str =
                        format!("aisuru:leaderboard:{}:{}", stat_obj.mode, user_country);

                    stat_obj.global_rank = Some(
                        redis_conn
                            .zrevrank(&leaderboard_str, &user_id)
                            .await
                            .unwrap_or(-1i32)
                            + 1,
                    );

                    stat_obj.country_rank = Some(
                        redis_conn
                            .zrevrank(&country_leaderboard_str, &user_id)
                            .await
                            .unwrap_or(-1i32)
                            + 1,
                    );

                    stats_vec.push(stat_obj);
                }
                _ => break,
            };
        }

        stats_vec
    }
}
