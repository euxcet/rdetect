use async_graphql::Context;

use crate::db::mongo::DataSource;
use crate::data::user::{self, model::User};
use crate::data::task::{self, model::Task};
use crate::util::constant::GqlResult;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn all_users(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db_detect.clone();
        user::service::all_users(db).await
    }

    async fn all_tasks(
        &self,
        ctx: &Context<'_>
    ) -> GqlResult<Vec<Task>> {
        let db = ctx.data_unchecked::<DataSource>().db_detect.clone();
        task::service::all_tasks(db).await
    }
}