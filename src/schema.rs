use juniper::FieldResult;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

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
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
