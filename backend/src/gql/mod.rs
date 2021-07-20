pub mod mutations;
pub mod queries;

use tide::{http::mime, Body, Request, Response, StatusCode};

use async_graphql::{
    http::{playground_source, receive_json, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};

use crate::State;

use crate::gql::queries::QueryRoot;

pub async fn build_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global mongodb datasource  in the schema object.
    // let mut schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription)
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub async fn graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    let gql_resp = schema.execute(receive_json(req).await?).await;

    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(Body::from_json(&gql_resp)?);

    Ok(resp.into())
}

pub async fn graphiql(_: Request<State>) -> tide::Result {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(playground_source(GraphQLPlaygroundConfig::new("graphql")));
    resp.set_content_type(mime::HTML);

    Ok(resp.into())
}