use crate::dto::game_dto::GameDto;
use crate::mapper::game_mapper;
use crate::service::game_service::GameService;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;

#[post("/game", format = "json", data = "<new_game>")]
pub async fn create_game(new_game: Json<GameDto>) -> Result<Json<GameDto>, Status> {
    let game_service = GameService::init().await;
    let game_entity = game_mapper::to_entity(new_game.into_inner()).await;
    let game_created = game_service.create_game(game_entity).await;
    match game_created {
        Ok(game_created) => {
            let game_output = game_mapper::to_dto(game_created).await;
            Ok(Json(game_output))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
