use crate::dto::game_dto::GameDto;
use crate::errors::game_service_error::{GameServiceError, GameServiceErrorKind};
use crate::mapper::game_mapper;
use crate::service::game_service::GameService;
use log::{debug, error};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};

#[post("/game", format = "json", data = "<new_game>")]
pub async fn create_game(new_game: Json<GameDto>) -> Result<Json<GameDto>, Status> {
    debug!("create_games resource started");
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
    debug!("create_games resource ending");
    result
}

#[get("/games", format = "json")]
pub async fn get_games() -> Result<Json<Vec<GameDto>>, Status> {
    debug!("get_games resource started");
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
    debug!("get_games resource ending");
    result
}

#[get("/game/<id>", format = "json")]
pub async fn get_game(id: String) -> Result<Json<GameDto>, Status> {
    debug!("get_game resource started");
    let game_service = GameService::init().await;
    let game_fetched = game_service.get_game(id).await;
    let result = match game_fetched {
        Ok(game_fetched) => Ok(Json(game_mapper::to_dto(game_fetched))),
        Err(err) => Err(process_service_error(err)),
    };
    debug!("get_game resource ending");
    result
}

fn process_service_error(error: GameServiceError) -> Status {
    error!("Error: {}", error.message);
    match error.kind {
        GameServiceErrorKind::NotFound => Status::NotFound,
        GameServiceErrorKind::Internal => Status::InternalServerError,
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::game_dto::GameDto;
    use crate::resource::game_resource::{create_game, get_game, get_games};
    use log::info;
    use rocket::async_test;
    use rocket::http::Status;
    use rocket::serde::json::Json;
    use serial_test::serial;
    use std::env;
    use testcontainers::clients::Cli;
    use testcontainers::GenericImage;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[async_test]
    #[serial]
    async fn create_game_should_insert_game_entity_and_return_created_game() {
        init();
        info!("Creating mongo container");
        let docker = Cli::default();
        let container = docker.run(GenericImage::new("mongo", "latest"));
        let port = container.get_host_port_ipv4(27017);
        let uri = format!("mongodb://localhost:{}", port);
        env::set_var("MONGO_URI", uri.clone());
        info!("Mongo container created");
        let new_game = GameDto {
            id: None,
            topics: vec!["Java".to_string()],
            question_number: 10,
            is_private: false,
        };
        let game_created = create_game(Json(new_game)).await.unwrap().into_inner();
        assert_eq!(game_created.id.is_some(), true);
        assert_eq!(game_created.topics, vec!["Java"]);
        assert_eq!(game_created.question_number, 10);
        assert_eq!(game_created.is_private, false);

        //Verify that the game was inserted in the DB
        let game_db = get_game(game_created.id.unwrap())
            .await
            .unwrap()
            .into_inner();
        assert_eq!(game_db.id.is_some(), true);
        assert_eq!(game_db.topics, vec!["Java"]);
        assert_eq!(game_db.question_number, 10);
        assert_eq!(game_db.is_private, false);
    }

    #[async_test]
    #[serial]
    async fn get_game_should_return_not_found_error() {
        init();
        info!("Creating mongo container");
        let docker = Cli::default();
        let container = docker.run(GenericImage::new("mongo", "latest"));
        let port = container.get_host_port_ipv4(27017);
        let uri = format!("mongodb://localhost:{}", port);
        env::set_var("MONGO_URI", uri.clone());
        info!("Mongo container created");
        let error = get_game("5f9e1b2a0b2b7c0009f9e1b9".to_string())
            .await
            .unwrap_err();
        assert_eq!(error, Status::NotFound);
    }

    #[async_test]
    #[serial]
    async fn get_game_should_return_db_connection_error() {
        init();
        info!("Creating mongo container");
        let docker = Cli::default();
        let container = docker.run(GenericImage::new("mongo", "latest"));
        let port = container.get_host_port_ipv4(27017);
        let uri = format!("mongodb://wronghost:{}", port);
        env::set_var("MONGO_URI", uri.clone());
        info!("Mongo container created");
        let error = get_game("5f9e1b2a0b2b7c0009f9e1b9".to_string())
            .await
            .unwrap_err();
        assert_eq!(error, Status::InternalServerError);
    }

    #[async_test]
    #[serial]
    async fn get_games_should_return_empty_result() {
        init();
        info!("Creating mongo container");
        let docker = Cli::default();
        let container = docker.run(GenericImage::new("mongo", "latest"));
        let port = container.get_host_port_ipv4(27017);
        let uri = format!("mongodb://localhost:{}", port);
        env::set_var("MONGO_URI", uri.clone());
        info!("Mongo container created");
        let games = get_games().await.unwrap().into_inner();
        assert_eq!(games.len(), 0);
    }

    #[async_test]
    #[serial]
    async fn get_games_should_return_the_created_games() {
        init();
        info!("Creating mongo container");
        let docker = Cli::default();
        let container = docker.run(GenericImage::new("mongo", "latest"));
        let port = container.get_host_port_ipv4(27017);
        let uri = format!("mongodb://localhost:{}", port);
        env::set_var("MONGO_URI", uri.clone());
        info!("Mongo container created");
        let new_game = GameDto {
            id: None,
            topics: vec!["Java".to_string()],
            question_number: 10,
            is_private: false,
        };
        info!("Creating game 1");
        let _ = create_game(Json(new_game.clone())).await;
        info!("Creating game 2");
        let _ = create_game(Json(new_game.clone())).await;
        info!("Get games");
        let games = get_games().await.unwrap().into_inner();
        assert_eq!(games.len(), 2);
    }
}
