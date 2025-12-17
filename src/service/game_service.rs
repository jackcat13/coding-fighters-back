use crate::errors::game_service_error::{GameServiceError, GameServiceErrorKind};
use crate::model::game::Game;
use crate::model::game_answer::GameAnswer;
use crate::model::game_progress::GameProgress;
use crate::repository::game_answer_repository::GameAnswerRepo;
use crate::repository::game_progress_repository::GameProgressRepo;
use crate::repository::game_repository::GameRepo;
use log::debug;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;
use rocket::futures::TryStreamExt;
use std::str::FromStr;

/// Service for [Game] object to interact with the data layer
pub struct GameService {
    game_repo: GameRepo,
    game_progress_repo: GameProgressRepo,
    game_answer_repo: GameAnswerRepo,
}

impl GameService {
    /// Creates a new instance of [GameService] with the repository to interact with the data layer
    pub async fn init() -> Self {
        let game_repo = GameRepo::init().await;
        let game_progress_repo = GameProgressRepo::init().await;
        let game_answer_repo = GameAnswerRepo::init().await;
        GameService {
            game_repo,
            game_progress_repo,
            game_answer_repo,
        }
    }

    /// Creates a new [Game].
    pub async fn create_game(&self, game: Game) -> Result<Game, GameServiceError> {
        debug!("create_games service started");
        let insert = self.game_repo.create_game(game.clone()).await;
        let result = match insert {
            Ok(insert) => {
                let mut game = game.clone();
                game.id = Some(insert.inserted_id.as_object_id().unwrap());
                Ok(game)
            }
            Err(err) => Err(Self::process_internal_error(err)),
        };
        debug!("create_games service ending");
        result
    }

    /// Gets all the [Game]s.
    pub async fn get_games(&self) -> Result<Vec<Game>, GameServiceError> {
        debug!("get_games service started");
        let result = match self.game_repo.get_games().await {
            Ok(mut games) => {
                let mut games_output = vec![];
                while let Some(game) = games.try_next().await.unwrap().or(None) {
                    games_output.push(game.clone());
                }
                Ok(games_output)
            }
            Err(err) => Err(Self::process_internal_error(err)),
        };
        debug!("get_games service ending");
        result
    }

    /// Gets a [Game] by id.
    /// Returns an error if the game does not exist.
    /// Returns an error if the id is not a valid ObjectId.
    pub async fn get_game(&self, id: String) -> Result<Game, GameServiceError> {
        debug!("get_game service started");
        let object_id =
            ObjectId::from_str(id.clone().as_str()).expect("Failed to create object id");
        let result = match self.game_repo.get_game(object_id).await {
            Ok(game) => match game {
                None => Err(Self::process_not_found_error(id)),
                Some(game) => Ok(game),
            },
            Err(err) => Err(Self::process_internal_error(err)),
        };
        debug!("get_game service ending");
        result
    }

    /// Patches a [Game] by id.
    /// Returns an error if the game does not exist.
    /// Returns an error if the id is not a valid ObjectId.
    pub async fn patch_game(&self, id: String) -> Result<String, GameServiceError> {
        debug!("patch_game service started");
        let object_id =
            ObjectId::from_str(id.clone().as_str()).expect("Failed to create object id");
        let result = match self.game_repo.patch_game(object_id).await {
            Ok(_) => Ok("".to_string()),
            Err(err) => Err(Self::process_internal_error(err)),
        };
        debug!("patch_game service ending");
        result
    }

    /// Saves the game
    pub async fn save_users_in_game(&self, game: &Game) {
        debug!("save_game service started");
        let _ = self.game_repo.save_users_in_game(game.clone()).await;
        debug!("save_game service ending");
    }

    /// Saves the game_progress of a particular game
    pub async fn save_game_progress(&self, game_progress: &GameProgress) {
        debug!("save_game_progress service started");
        let _ = self
            .game_progress_repo
            .save_game_progress(game_progress.clone())
            .await;
        debug!("save_game_progress service ending");
    }

    /// Replaces the game_progress of a particular game
    pub async fn replace_game_progress(&self, game_progress: &GameProgress) {
        debug!("replace_game_progress service started");
        let _ = self
            .game_progress_repo
            .replace_game_progress(game_progress.clone())
            .await;
        debug!("replace_game_progress service ending");
    }

    /// Gets the game_progress of a particular game
    pub async fn get_game_progress(&self, id: String) -> Result<GameProgress, GameServiceError> {
        debug!("get_game_progress service started");
        let result = match self.game_progress_repo.get_game_progress(id.clone()).await {
            Ok(progress) => match progress {
                None => Err(Self::process_not_found_error(id)),
                Some(progress) => Ok(progress),
            },
            Err(err) => Err(Self::process_internal_error(err)),
        };
        debug!("get_game_progress service ending");
        result
    }

    /// Gets the game answers of a particular game
    pub async fn save_game_answer(&self, game_answer: &GameAnswer) {
        debug!("save_game_answers service started");
        let _ = self
            .game_answer_repo
            .save_game_answer(game_answer.clone())
            .await;
        debug!("save_game_answers service ending");
    }

    /// Gets a game result
    pub async fn get_game_result(&self, id: String) -> Result<Vec<GameAnswer>, GameServiceError> {
        debug!("get_game_result service started");
        let result = match self.game_answer_repo.get_game_answers(id).await {
            Ok(mut answers) => {
                let mut answers_output = vec![];
                while let Some(answer) = answers.try_next().await.unwrap().or(None) {
                    answers_output.push(answer.clone());
                }
                Ok(answers_output)
            }
            Err(err) => Err(Self::process_internal_error(err)),
        };
        debug!("get_game_result service ending");
        result
    }

    fn process_not_found_error(id: String) -> GameServiceError {
        GameServiceError {
            message: format!("Game with id {} does not exist", id),
            kind: GameServiceErrorKind::NotFound,
        }
    }

    fn process_internal_error(err: Error) -> GameServiceError {
        GameServiceError {
            message: err.to_string(),
            kind: GameServiceErrorKind::Internal,
        }
    }
}
