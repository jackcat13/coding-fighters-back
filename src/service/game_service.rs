use crate::model::game::Game;
use crate::repository::game_repository::GameRepo;
use mongodb::error::Error;

pub struct GameService {
    game_repo: GameRepo,
}

impl GameService {
    pub async fn init() -> Self {
        let game_repo = GameRepo::init().await;
        GameService { game_repo }
    }

    pub async fn create_game(&self, game: Game) -> Result<Game, Error> {
        let insert = self.game_repo.create_user(game.clone()).await;
        match insert {
            Ok(_) => Ok(game),
            Err(err) => Err(err),
        }
    }
}
