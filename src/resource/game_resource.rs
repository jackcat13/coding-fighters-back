use crate::dto::game_dto::GameDto;
use crate::dto::game_progress_dto::{questions, GameProgressDto};
use crate::errors::game_service_error::{GameServiceError, GameServiceErrorKind};
use crate::mapper::game_mapper;
use crate::mapper::game_mapper::progress_to_entity;
use crate::service::game_service::GameService;
use log::{debug, error, info};
use rand::Rng;
use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::{task, time};
use rocket::{get, patch, post};
use std::time::Duration;

/// POST request to create a new game.
/// Returns the created game.
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

/// GET request to get all the games.
/// Returns a list of games.
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

/// GET request to get a game by id.
/// Returns the game.
/// Returns an error if the game does not exist.
/// Returns an error if the id is not a valid ObjectId.
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

/// GET request to get a game progress.
/// Returns events to sync game progress with clients.
/// Returns an error if the game does not exist.
/// Returns an error if the id is not a valid ObjectId.
#[get("/game/<id>/progress")]
pub async fn game_progress(id: String) -> EventStream![] {
    EventStream! {
        debug!("game_progress events started");
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            match get_game(id.clone()).await {
                Ok(result) => if result.is_started {break},
                Err(_) => error!("Problem occurred when fetching game in sse game progress"),
            }
            yield Event::data("NOT STARTED");
            interval.tick().await;
        }
        loop {
            let game_service = GameService::init().await;
            match game_service.get_game_progress(id.clone()).await {
                Ok(result) => {
                    yield Event::json(&result);
                },
                Err(_) => error!("Problem occurred when fetching game in sse game progress"),
            }
            interval.tick().await;
        }
    }
}

/// PATCH request to update a game content.
/// Returns the game.
/// Returns an error if the game does not exist.
/// Returns an error if the id is not a valid ObjectId.
#[patch("/game/<id>", format = "json")]
pub async fn patch_game(id: String) -> Result<Json<String>, Status> {
    debug!("patch_game resource started");
    let game_service = GameService::init().await;
    let game_fetched = game_service.patch_game(id.clone()).await;
    let result = match game_fetched {
        Ok(_) => {
            task::spawn(async move { start_new_game(id.clone()).await });
            Ok(Json("".to_string()))
        }
        Err(err) => Err(process_service_error(err)),
    };
    debug!("patch_game resource ending");
    result
}

async fn start_new_game(id: String) {
    info!("Starting the game");
    let questions = questions();
    let game_service = GameService::init().await;
    let game = game_service.get_game(id.clone()).await;
    if let Ok(game) = game {
        let random_index = rand::thread_rng().gen_range(0..questions.len());
        let question = questions.get(random_index).unwrap().clone();
        let mut game_proress_dto = GameProgressDto {
            game_id: game.id.unwrap().to_string(),
            current_question: 1,
            question_number: game.question_number,
            question_content: question.clone(),
        };
        let game_progress_entity = progress_to_entity(game_proress_dto.clone());
        game_service.save_game_progress(&game_progress_entity).await;
        let mut interval = time::interval(Duration::from_secs(20));
        for _ in 1..game_proress_dto.question_number {
            info!("Next question");
            let game_progress_entity = progress_to_entity(game_proress_dto.clone());
            interval.tick().await;
            game_proress_dto.question_number += 1;
            let random_index = rand::thread_rng().gen_range(0..questions.len());
            game_proress_dto.question_content =
                questions.get(random_index).unwrap().clone().clone();
            game_service
                .replace_game_progress(&game_progress_entity)
                .await;
        }
    }
    info!("End of the game");
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
    use crate::resource::game_resource::{create_game, get_game, get_games, patch_game};
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
            is_started: false,
            creator: Some("bob".to_string()),
        };
        let game_created = create_game(Json(new_game)).await.unwrap().into_inner();
        assert_eq!(game_created.id.is_some(), true);
        assert_eq!(game_created.topics, vec!["Java"]);
        assert_eq!(game_created.question_number, 10);
        assert_eq!(game_created.is_private, false);
        assert_eq!(game_created.is_started, false);
        assert_eq!(game_created.creator, Some("bob".to_string()));

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
            is_started: false,
            creator: Some("bob".to_string()),
        };
        info!("Creating game 1");
        let _ = create_game(Json(new_game.clone())).await;
        info!("Creating game 2");
        let _ = create_game(Json(new_game.clone())).await;
        info!("Get games");
        let games = get_games().await.unwrap().into_inner();
        assert_eq!(games.len(), 2);
    }

    #[async_test]
    #[serial]
    async fn get_games_should_return_only_public_and_not_started_games() {
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
            is_started: false,
            creator: Some("bob".to_string()),
        };
        let new_game_private = GameDto {
            id: None,
            topics: vec!["Java".to_string()],
            question_number: 10,
            is_private: true,
            is_started: false,
            creator: Some("bob".to_string()),
        };
        let new_game_started = GameDto {
            id: None,
            topics: vec!["Java".to_string()],
            question_number: 10,
            is_private: false,
            is_started: true,
            creator: Some("bob".to_string()),
        };
        info!("Creating game 1");
        let _ = create_game(Json(new_game.clone())).await;
        info!("Creating game 2");
        let _ = create_game(Json(new_game.clone())).await;
        info!("Creating game 3");
        let _ = create_game(Json(new_game_private.clone())).await;
        info!("Creating game 4");
        let _ = create_game(Json(new_game_private.clone())).await;
        info!("Creating game 5");
        let _ = create_game(Json(new_game.clone())).await;
        info!("Creating game 6");
        let _ = create_game(Json(new_game_started.clone())).await;
        info!("Get games");
        let games = get_games().await.unwrap().into_inner();
        assert_eq!(games.len(), 3);
    }

    #[async_test]
    #[serial]
    async fn patch_game_should_modify_existing_game() {
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
            is_started: false,
            creator: Some("bob".to_string()),
        };
        let game_created = create_game(Json(new_game)).await.unwrap().into_inner();
        assert_eq!(game_created.is_started, false);

        //Update the game
        let game_id = game_created.id.expect("Failed to get game id");
        let _ = GameDto {
            id: Some(game_id.clone()),
            topics: vec!["Java".to_string()],
            question_number: 10,
            is_private: false,
            is_started: true,
            creator: Some("bob".to_string()),
        };
        let _ = patch_game(game_id.clone()).await;

        //Verify that the game was inserted in the DB
        let game_db = get_game(game_id).await.unwrap().into_inner();
        assert_eq!(game_db.is_started, true);
    }
}
