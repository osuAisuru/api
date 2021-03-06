use juniper::FieldResult;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

use crate::models::leaderboard::Leaderboard;
use crate::models::online::OnlineInfo;
use crate::models::scores::Scores;
use crate::models::stats::Stats;
use crate::models::user::User;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn user(id: Option<i32>, name: Option<String>) -> FieldResult<User> {
        let user_opt = match (id, name) {
            (Some(id), None) => User::from_user_id(id).await,
            (None, Some(name)) => User::from_user_name(name).await,
            _ => return FieldResult::Err("id or name must be specified".into()),
        };

        let mut user = match user_opt {
            Some(user) => user,
            _ => return FieldResult::Err("user not found".into()),
        };

        let stats_vec = Stats::from_user_id(user.id, user.country.clone()).await;
        user.stats = Some(stats_vec);

        FieldResult::Ok(user)
    }

    async fn user_scores(id: i32, mode: i32) -> FieldResult<Scores> {
        let scores = Scores::from_user_id(id, mode).await;
        FieldResult::Ok(scores)
    }

    async fn leaderboard(mode: i32, country: Option<String>) -> FieldResult<Leaderboard> {
        let leaderboard = match country {
            Some(country) => Leaderboard::for_mode_country(mode, country).await,
            None => Leaderboard::for_mode(mode).await,
        };

        FieldResult::Ok(leaderboard)
    }

    async fn user_search(query: String) -> FieldResult<Vec<User>> {
        let users = User::search(query).await;
        FieldResult::Ok(users)
    }

    async fn online_info() -> FieldResult<OnlineInfo> {
        let online_info = OnlineInfo::get_online_info().await;
        FieldResult::Ok(online_info)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
