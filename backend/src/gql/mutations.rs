use async_graphql::Context;

use crate::db::mongo::DataSource;
use crate::data::task::{self, model::{Task, NewTask}};
use crate::util::constant::GqlResult;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn new_task(
        &self,
        ctx: &Context<'_>,
        new_task: NewTask,
    ) -> GqlResult<Task> {
        let db = ctx.data_unchecked::<DataSource>().db_detect.clone();
        task::service::new_task(db, new_task).await
    }
}