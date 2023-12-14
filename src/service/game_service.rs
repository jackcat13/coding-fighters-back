use crate::model::game::Game;
use crate::repository::game_repository::GameRepo;
use mongodb::error::Error;
use rocket::futures::TryStreamExt;

pub struct GameService {
    game_repo: GameRepo,
}

impl GameService {
    pub async fn init() -> Self {
        let game_repo = GameRepo::init().await;
        GameService { game_repo }
    }

    pub async fn create_game(&self, game: Game) -> Result<Game, Error> {
        let insert = self.game_repo.create_game(game.clone()).await;
        match insert {
            Ok(_) => Ok(game),
            Err(err) => Err(err),
        }
    }

    pub async fn get_games(&self) -> mongodb::error::Result<Vec<Game>> {
        match self.game_repo.get_games().await {
            Ok(mut games) => {
                let mut games_output = vec![];
                while let Some(game) = games.try_next().await? {
                    games_output.push(game.clone());
                }
                Ok(games_output)
            }
            Err(err) => Err(err),
        }
    }
}
