use crate::dto::game_dto::GameDto;
use crate::mapper::game_mapper;
use crate::service::game_service::GameService;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use crate::errors::game_service_error::{GameServiceError, GameServiceErrorKind};

#[post("/game", format = "json", data = "<new_game>")]
pub async fn create_game(new_game: Json<GameDto>) -> Result<Json<GameDto>, Status> {
    println!("create_games resource started");
    let game_service = GameService::init().await;
    let game_entity = game_mapper::to_entity(new_game.into_inner());
    let game_created = game_service.create_game(game_entity).await;
    let result = match game_created {
        Ok(game_created) => {
            let game_output = game_mapper::to_dto(game_created);
            Ok(Json(game_output))
        }
        Err(err) => Err(process_service_error(err)),
    };
    println!("create_games resource ending");
    result
}

#[get("/games", format = "json")]
pub async fn get_games() -> Result<Json<Vec<GameDto>>, Status> {
    println!("get_games resource started");
    let game_service = GameService::init().await;
    let games_fetched = game_service.get_games().await;
    let result = match games_fetched {
        Ok(games_fetched) => {
            let game_output: Vec<GameDto> = games_fetched
                .iter()
                .map(|game| game_mapper::to_dto(game.clone()))
                .collect();
            Ok(Json(game_output))
        }
        Err(err) => Err(process_service_error(err)),
    };
    println!("get_games resource ending");
    result
}

#[get("/game/<id>", format = "json")]
pub async fn get_game(id: String) -> Result<Json<GameDto>, Status> {
    println!("get_game resource started");
    let game_service = GameService::init().await;
    let game_fetched = game_service.get_game(id).await;
    let result = match game_fetched {
        Ok(game_fetched) => Ok(Json(game_mapper::to_dto(game_fetched))),
        Err(err) => Err(process_service_error(err)),
    };
    println!("get_game resource ending");
    result
}

fn process_service_error(error: GameServiceError) -> Status {
    println!("Error: {}", error.message);
    match error.kind {
        GameServiceErrorKind::NotFound => Status::NotFound,
        GameServiceErrorKind::Internal => Status::InternalServerError,
    }
}
