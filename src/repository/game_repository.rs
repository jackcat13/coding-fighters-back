use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection, Cursor};
use std::env;
extern crate dotenv;
use crate::model::game::Game;
use dotenv::dotenv;
use log::{debug, info};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
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
        debug!("Creating DB client");
        let client = Client::with_options(client_options).expect("Failed to create mongo client");
        debug!("Accessing DB");
        let db = client.database(MONGO_DATABASE);
        debug!("Accessing collection");
        let col: Collection<Game> = db.collection(GAME);
        debug!("DB client created");
        GameRepo { col }
    }

    pub async fn create_game(&self, new_game: Game) -> mongodb::error::Result<InsertOneResult> {
        debug!("Creating game in DB");
        let game_created = self.col.insert_one(new_game, None).await;
        info!("Game created in DB");
        game_created
    }

    pub async fn get_games(&self) -> mongodb::error::Result<Cursor<Game>> {
        debug!("Getting games from DB");
        let games = self.col.find(None, None).await;
        info!("Games retrieved from DB");
        games
    }

    pub async fn get_game(&self, id: ObjectId) -> mongodb::error::Result<Option<Game>> {
        debug!("Getting game by id from DB");
        let game = self.col.find_one(doc! {"_id": id}, None).await;
        info!("Game retrieved by id from DB");
        game
    }
}
