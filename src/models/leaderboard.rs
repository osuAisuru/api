use juniper::GraphQLObject;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use super::stats::Stats;
use super::user::User;

use crate::REDIS;

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct LeaderboardUser {
    pub user: User,
    pub stats: Stats,
}

#[derive(Clone, GraphQLObject, Deserialize, Serialize, Debug)]
pub struct Leaderboard {
    pub mode: i32,
    pub users: Vec<LeaderboardUser>,
}

impl Leaderboard {
    pub async fn for_mode(mode: i32) -> Self {
        let mut redis_conn = REDIS.get().unwrap().get_async_connection().await.unwrap();

        let leaderboard_str = format!("aisuru:leaderboard:{}", mode);
        let users: Vec<i32> = redis_conn
            .zrevrangebyscore(&leaderboard_str, "+inf", "-inf")
            .await
            .unwrap();

        let mut users_vec: Vec<LeaderboardUser> = Vec::new();

        for user_id in users {
            let user = User::from_user_id(user_id).await;

            if user.is_none() {
                continue;
            }

            let user = user.unwrap();
            let user_stats = Stats::from_user_id(user_id, user.country.clone()).await;
            let mode_stat: &Stats = user_stats.get(mode as usize).unwrap();

            let leaderboard_user = LeaderboardUser {
                user: user.clone(),
                stats: mode_stat.clone(),
            };

            users_vec.push(leaderboard_user);
        }

        Self {
            mode,
            users: users_vec,
        }
    }

    pub async fn for_mode_country(mode: i32, country: String) -> Self {
        let mut redis_conn = REDIS.get().unwrap().get_async_connection().await.unwrap();

        let leaderboard_str = format!("aisuru:leaderboard:{}:{}", mode, country);
        let users: Vec<i32> = redis_conn
            .zrevrangebyscore(&leaderboard_str, "+inf", "-inf")
            .await
            .unwrap();

        let mut users_vec: Vec<LeaderboardUser> = Vec::new();

        for user_id in users {
            let user = User::from_user_id(user_id).await;

            if user.is_none() {
                continue;
            }

            let user = user.unwrap();
            let user_stats = Stats::from_user_id(user_id, user.country.clone()).await;
            let mode_stat: &Stats = user_stats.get(mode as usize).unwrap();

            let leaderboard_user = LeaderboardUser {
                user: user.clone(),
                stats: mode_stat.clone(),
            };

            users_vec.push(leaderboard_user);
        }

        Self {
            mode,
            users: users_vec,
        }
    }
}
