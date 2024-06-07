use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection};
use std::env;
extern crate dotenv;
use crate::model::game_progress::GameProgress;
use dotenv::dotenv;
use log::{debug, info};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;

pub const MONGO_URI: &str = "MONGO_URI";
pub const MONGO_DATABASE: &str = "CodingFighters";
pub const GAME_PROGRESS: &str = "GameProgress";

/// Repository for [Game] object to interact with the database
pub struct GameProgressRepo {
    col: Collection<GameProgress>,
}

impl GameProgressRepo {
    /// Creates a new instance of [GameProgressRepo] with the collection to interact with the database
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var(MONGO_URI) {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable".to_string(),
        };
        let client_options = ClientOptions::parse(uri)
            .await
            .expect("Failed to create client options");
        debug!("Creating DB client");
        let client = Client::with_options(client_options).expect("Failed to create mongo client");
        debug!("Accessing DB");
        let db = client.database(MONGO_DATABASE);
        debug!("Accessing collection");
        let col: Collection<GameProgress> = db.collection(GAME_PROGRESS);
        debug!("DB client created");
        GameProgressRepo { col }
    }

    /// Saves a [GameProgress] in the database.
    pub async fn save_game_progress(
        &self,
        new_game_progress: GameProgress,
    ) -> mongodb::error::Result<InsertOneResult> {
        debug!("Saving game progress in DB");
        let game_progress_saved = self.col.insert_one(new_game_progress, None).await;
        info!("Game progress saved in DB");
        game_progress_saved
    }

    /// Replaces a [GameProgress] in the database.
    pub async fn replace_game_progress(
        &self,
        new_game_progress: GameProgress,
    ) -> mongodb::error::Result<Option<GameProgress>> {
        debug!("Replace game progress in DB");
        let id = new_game_progress.clone().id;
        let game_progress_saved = self
            .col
            .find_one_and_replace(doc! {"_id": id}, new_game_progress, None)
            .await;
        info!("Game progress replaced in DB");
        game_progress_saved
    }

    /// Gets a [GameProgress] in the database.
    pub async fn get_game_progress(
        &self,
        id: String,
    ) -> mongodb::error::Result<Option<GameProgress>> {
        debug!("Getting game progress in DB");
        let game_progress_saved = self.col.find_one(doc! {"_id": id}, None).await;
        info!("Game progress returned from in DB");
        game_progress_saved
    }
}
