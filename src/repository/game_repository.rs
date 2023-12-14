use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection, Cursor};
use std::env;
extern crate dotenv;
use crate::model::game::Game;
use dotenv::dotenv;

pub const MONGO_URI: &str = "MONGO_URI";
pub const MONGO_DATABASE: &str = "CoddingFighters";
pub const GAME: &str = "Game";

pub struct GameRepo {
    col: Collection<Game>,
}

impl GameRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var(MONGO_URI) {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable".to_string(),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("Failed to create mongo client");
        let db = client.database(MONGO_DATABASE);
        let col: Collection<Game> = db.collection(GAME);
        GameRepo { col }
    }

    pub async fn create_game(&self, new_game: Game) -> mongodb::error::Result<InsertOneResult> {
        self.col.insert_one(new_game, None).await
    }

    pub async fn get_games(&self) -> mongodb::error::Result<Cursor<Game>> {
        self.col.find(None, None).await
    }
}
