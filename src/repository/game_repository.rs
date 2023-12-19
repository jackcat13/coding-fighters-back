use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection, Cursor};
use std::env;
extern crate dotenv;
use crate::model::game::Game;
use dotenv::dotenv;
use mongodb::options::ClientOptions;

pub const MONGO_URI: &str = "MONGO_URI";
pub const MONGO_DATABASE: &str = "CodingFighters";
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
        let client_options = ClientOptions::parse(uri)
            .await
            .expect("Failed to create client options");
        println!("Creating DB client");
        let client = Client::with_options(client_options).expect("Failed to create mongo client");
        println!("Accessing DB");
        let db = client.database(MONGO_DATABASE);
        println!("Accessing collection");
        let col: Collection<Game> = db.collection(GAME);
        println!("DB client created");
        GameRepo { col }
    }

    pub async fn create_game(&self, new_game: Game) -> mongodb::error::Result<InsertOneResult> {
        println!("Creating game in DB");
        let game_created = self.col.insert_one(new_game, None).await;
        println!("Game created in DB");
        game_created
    }

    pub async fn get_games(&self) -> mongodb::error::Result<Cursor<Game>> {
        println!("Getting games from DB");
        let games = self.col.find(None, None).await;
        println!("Games retrieved from DB");
        games
    }
}
