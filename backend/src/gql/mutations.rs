use async_graphql::{Context, Upload, ID};

use futures::lock::Mutex;
use slab::Slab;

use crate::db::mongo::DataSource;
use crate::data::task::{self, model::{Task, NewTask}};
use crate::util::constant::GqlResult;

pub struct MutationRoot;

#[derive(Clone, async_graphql::SimpleObject)]
pub struct FileInfo {
    id: ID,
    filename: String,
    mimetype: Option<String>,
}

pub type Storage = Mutex<Slab<FileInfo>>;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        new_task: NewTask,
    ) -> GqlResult<Task> {
        let db = ctx.data_unchecked::<DataSource>().db_detect.clone();
        task::service::create_task(db, new_task).await
    }

    async fn single_upload(
        &self,
        ctx: &Context<'_>,
        file: Upload
    ) -> FileInfo {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        println!("files count: {}", storage.len());
        let entry = storage.vacant_entry();
        let upload = file.value(ctx).unwrap();
        let info = FileInfo {
            id: entry.key().into(),
            filename: upload.filename.clone(),
            mimetype: upload.content_type,
        };
        entry.insert(info.clone());
        info
    }
}