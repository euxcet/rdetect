use async_graphql::{Error, ErrorExtensions};
use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{Bson, Document, doc, to_bson, from_bson}
};

use crate::data::task::model::{Task, NewTask};
use crate::util::constant::GqlResult;

pub async fn all_tasks(db: Database) -> GqlResult<Vec<Task>> {
    let coll = db.collection("tasks");

    let mut tasks: Vec<Task> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let task = from_bson(Bson::Document(document)).unwrap();
                tasks.push(task);
            }
            Err(error) => Err(Error::new("1-all-tasks")
                .extend_with(|_, e| e.set("details", format!("Error to find doc: {}", error))))
            .unwrap(),
        }
    }

    if tasks.len() > 0 {
        Ok(tasks)
    } else {
        Err(Error::new("1-all-tasks").extend_with(|_, e| e.set("details", "No records")))
    }
}

pub async fn get_task_by_name(db: Database, name: &str) -> GqlResult<Task> {
    let coll = db.collection("tasks");

    let exist_document = coll.find_one(doc! {"name": name}, None).await;

    if let Ok(task_document) = exist_document {
        if let Some(document) = task_document {
            let task: Task = from_bson(Bson::Document(document)).unwrap();
            Ok(task)
        } else {
            Err(Error::new("2-name").extend_with(|_, e| e.set("details", "Task name doesn't exist.")))
        }
    } else {
        Err(Error::new("2-name").extend_with(|_, e| e.set("details", "Failed to search mongodb.")))
    }
}

pub async fn create_task(db: Database, mut new_task: NewTask) -> GqlResult<Task> {
    let coll = db.collection("tasks");
    if self::get_task_by_name(db.clone(), &new_task.name).await.is_ok() {
        Err(Error::new("email-exists"))
    } else {
        let new_task_bson = to_bson(&new_task).unwrap();
        if let Bson::Document(document) = new_task_bson {
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert task into mongodb.");
            self::get_task_by_name(db.clone(), &new_task.name).await
        } else {
            Err(Error::new("3-new_task"))
        }
    }
}