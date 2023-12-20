use crate::model::game::Game;
use crate::repository::game_repository::GameRepo;
use mongodb::bson::oid::ObjectId;
use rocket::futures::TryStreamExt;
use std::str::FromStr;

pub struct GameService {
    game_repo: GameRepo,
}

impl GameService {
    pub async fn init() -> Self {
        let game_repo = GameRepo::init().await;
        GameService { game_repo }
    }

    pub async fn create_game(&self, game: Game) -> Result<Game, mongodb::error::Error> {
        println!("create_games service started");
        let insert = self.game_repo.create_game(game.clone()).await;
        let result = match insert {
            Ok(insert) => {
                let mut game = game.clone();
                game.id = Some(insert.inserted_id.as_object_id().unwrap());
                Ok(game)
            }
            Err(err) => Err(err),
        };
        println!("create_games service ending");
        result
    }

    pub async fn get_games(&self) -> mongodb::error::Result<Vec<Game>> {
        println!("get_games service started");
        let result = match self.game_repo.get_games().await {
            Ok(mut games) => {
                let mut games_output = vec![];
                while let Some(game) = games.try_next().await? {
                    games_output.push(game.clone());
                }
                Ok(games_output)
            }
            Err(err) => Err(err),
        };
        println!("get_games service ending");
        result
    }

    pub async fn get_game(&self, id: String) -> Result<Game, String> {
        println!("get_game service started");
        let object_id =
            ObjectId::from_str(id.clone().as_str()).expect("Failed to create object id");
        let result = match self.game_repo.get_game(object_id).await {
            Ok(game) => match game {
                None => Err(format!("Game with id {} does not exist", id)),
                Some(game) => Ok(game),
            },
            Err(_) => Err("An error occurred.".to_string()),
        };
        println!("get_game service ending");
        result
    }
}
