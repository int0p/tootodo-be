pub mod error;
use crate::config::Config;
use error::{Error, Result};
use mongodb::Database;
use mongodb::{options::ClientOptions, Client};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct DB {
    pub db: Pool<Postgres>,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let config = Config::init();
        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await
        {
            Ok(pool) => {
                println!("✅ Connection to the database is successful!");
                pool
            }
            Err(e) => {
                e.to_string();
                std::process::exit(1);
            }
        };

        match sqlx::migrate!("./migrations").run(&pool).await {
            Ok(_) => println!("✅ Migrations executed successfully."),
            Err(e) => {
                e.to_string();
            }
        };

        Ok(Self { db: pool })
    }
}

#[derive(Clone, Debug)]
pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn init() -> Result<Self> {
        let config = Config::init();
        let mongodb_uri = config.mongodb_url;
        let database_name = config.mongo_initdb_db;

        let mut client_options = ClientOptions::parse(mongodb_uri)
            .await
            .map_err(Error::MongoError)?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options).map_err(Error::MongoError)?;
        let db = client.database(database_name.as_str());

        // let collection_name = config.mongo_collection_note;
        // let note_collection = database.collection::<NoteModel>(collection_name.as_str());
        // let note_doc = database.collection::<Document>(collection_name.as_str());
        // let note = MemoBMC { collection: note_collection, doc_collection: note_doc};

        println!("✅ Mongo Database connected successfully");

        Ok(Self { db })
    }

    pub async fn init_test() -> Result<Self> {
        let config = Config::init();
        let mongodb_url = config.mongodb_test_url;
        let database_name = config.mongo_test_db;

        let mut client_options = ClientOptions::parse(mongodb_url)
            .await
            .map_err(Error::MongoError)?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options).map_err(Error::MongoError)?;
        let db = client.database(database_name.as_str());

        // println!("✅ Mongo <Test> Database connected successfully");

        Ok(Self { db })
    }
}
