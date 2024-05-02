pub mod error;
use crate::models::memo::controller::MemoBMC;
use crate::models::memo::model::NoteModel;
use crate::config::{self, Config};
use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client, Collection};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use error::{Error,Result};

pub struct DB{
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
            Error::FailToCreatePool(e.to_string());
            std::process::exit(1);
        }
    };

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("✅ Migrations executed successfully."),
        Err(e) => {Error::MigrationError(e.to_string());},
    };

    Ok(Self {
        db: pool
    })
    }
}

#[derive(Clone, Debug)]
pub struct MongoDB {
    pub note: MemoBMC,
}

impl MongoDB {
    pub async fn init() -> Result<Self> {
        let config = Config::init();
        let mongodb_uri = config.mongodb_url;
        let database_name = config.mongo_initdb_db;
        let collection_name = config.mongo_collection_note;

        let mut client_options = ClientOptions::parse(mongodb_uri).await.map_err(Error::MongoError)?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options).map_err(Error::MongoError)?;
        let database = client.database(database_name.as_str());

        let note_collection = database.collection::<NoteModel>(collection_name.as_str());
        let note_doc = database.collection::<Document>(collection_name.as_str());
        let note = MemoBMC { collection: note_collection, doc_collection: note_doc};

        println!("✅ Mongo Database connected successfully");

        Ok(Self {
            note,
        })
    }
}
