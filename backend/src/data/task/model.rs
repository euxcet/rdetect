use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub _id: ObjectId,
    pub name: String,
    pub image_folder: String,
    pub xml_folder: String,
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct NewTask {
    pub name: String,
    pub image_folder: String,
    pub xml_folder: String,
    pub labels: Vec<String>,
}