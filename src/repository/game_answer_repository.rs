extern crate dotenv;
use dotenv::dotenv;
use log::debug;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Cursor};
use std::env;

use crate::model::game_answer::GameAnswer;

pub const MONGO_URI: &str = "MONGO_URI";
pub const MONGO_DATABASE: &str = "CodingFighters";
pub const GAME_ANSWER: &str = "GameAnswer";

/// Repository for [GameAnswer] object to interact with the database
pub struct GameAnswerRepo {
    col: Collection<GameAnswer>,
}

impl GameAnswerRepo {
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
        let col: Collection<GameAnswer> = db.collection(GAME_ANSWER);
        debug!("DB client created");
        GameAnswerRepo { col }
    }

    /// Saves a [GameAnswer] in the database.
    pub async fn save_game_answer(
        &self,
        new_game_answer: GameAnswer,
    ) -> mongodb::error::Result<mongodb::results::InsertOneResult> {
        debug!("Saving game answer in DB");
        let game_answer_saved = self.col.insert_one(new_game_answer, None).await;
        debug!("Game answer saved in DB");
        game_answer_saved
    }

    /// Gets all the [Answer]s of a game from the database.
    /// Returns an empty list if there are no answers.
    pub async fn get_game_answers(&self, id: String) -> mongodb::error::Result<Cursor<GameAnswer>> {
        debug!("Getting answers from DB");
        let games = self.col.find(doc!("game_id": id), None).await;
        debug!("Games retrieved from DB");
        games
    }
}
