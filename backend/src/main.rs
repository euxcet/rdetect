mod gql;

use crate::gql::{build_schema, graphiql, graphql};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // 初始 Tide 应用程序状态
    let schema = build_schema().await;
    let app_state = State { schema: schema };
    let mut app = tide::with_state(app_state);

    // 路由配置
    app.at("graphql").post(graphql);
    app.at("graphiql").get(graphiql);

    app.listen(format!("{}:{}", "127.0.0.1", "8080")).await?;

    Ok(())
}

//  Tide 应用程序作用域状态 state.
#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    >,
}