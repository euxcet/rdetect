use crate::util::constant::CFG;

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
            ClientOptions::parse(CFG.get("MONGODB_URI").unwrap())
                .await
                .expect("Failed to parse options!");
                
        let client = Client::with_options(client_options)
            .expect("Failed to initialize database!");


        let db_detect = client.database(CFG.get("MONGODB_DATABASE").unwrap());

        DataSource { client: client, db_detect: db_detect }
    }
}