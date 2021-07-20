mod gql;
mod db;
mod data;
mod util;

use crate::gql::{build_schema, graphiql, graphql};
use crate::util::constant::CFG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    let schema = build_schema().await;
    let app_state = State { schema: schema };
    let mut app = tide::with_state(app_state);

    app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(graphql);
    app.at(CFG.get("GRAPHIQL_PATH").unwrap()).get(graphiql);

    app.listen(format!("{}:{}", "127.0.0.1", "8080")).await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}