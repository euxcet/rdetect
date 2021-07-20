use mongodb::{Client, options::ClientOptions, Database};

pub struct DataSource {
    client: Client,
    pub db_detect: Database,
}

#[allow(dead_code)]
impl DataSource {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> DataSource {
        let mut client_options =
            ClientOptions::parse("mongodb://mongo:mongo@localhost:27017")
                .await
                .expect("Failed to parse options!");
        client_options.app_name = Some("tide-graphql-mongodb".to_string());

        let client = Client::with_options(client_options)
            .expect("Failed to initialize database!");

        let db_detect = client.database("detect");

        DataSource { client: client, db_detect: db_detect }
    }
}