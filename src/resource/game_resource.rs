use crate::dto::answer::GameAnswerDto;
use crate::dto::game_dto::GameDto;
use crate::dto::game_progress_dto::{
    questions_java, questions_kotlin, questions_rust, GameProgressDto, QuestionDto,
};
use crate::errors::game_service_error::{GameServiceError, GameServiceErrorKind};
use crate::mapper::game_mapper::{self, answer_to_entity};
use crate::mapper::game_mapper::{entity_to_progress, progress_to_entity};
use crate::mapper::question_mapper;
use crate::model::game::Game;
use crate::service::game_service::GameService;
use log::{debug, error, info};
use rand::Rng;
use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::{task, time};
use rocket::{get, patch, post};
use std::time::Duration;
use std::vec;

pub const QUESTION_SECONDS: u64 = 20;

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
            let mut users: Vec<String> = vec![];
            match get_game(id.clone()).await {
                Ok(result) => {
                    users = result.users.clone();
                    if result.is_started {break}
                },
                Err(_) => error!("Problem occurred when fetching game in sse game progress"),
            }
            let users_string = users.iter().map(|u| u.to_owned() + "\n").collect::<String>();
            yield Event::data("NOT STARTED".to_owned() + users_string.as_str());
            interval.tick().await;
        }
        let game_service = GameService::init().await;
        let mut current = game_service.get_game_progress(id.clone()).await.expect("Failed to get game progress");
        while current.current_question < current.question_number && current.question_content.remaining_time > 0 {
            let game_service = GameService::init().await;
            match game_service.get_game_progress(id.clone()).await {
                Ok(result) => {
                    current = result.clone();
                    let game_progress_dto = entity_to_progress(result);
                    yield Event::json(&game_progress_dto);
                },
                Err(_) => error!("Problem occurred when fetching game in sse game progress"),
            }
            interval.tick().await;
        }
        yield Event::data("END");
        return;
    }
}

/// POST request to save resonse of a player
#[post("/game/<id>/progress/<answer>", format = "json", data = "<user>")]
pub async fn game_progress_answer(id: String, answer: i8, user: String) {
    debug!("game_progress_answer started");
    let game_service = GameService::init().await;
    let game_progress = game_service
        .get_game_progress(id.clone())
        .await
        .expect("Failed to get game progress");
    let answer = GameAnswerDto {
        game_id: id,
        user,
        answer,
        question_index: game_progress.current_question,
        correct_answer: game_progress.question_content.good_answer_number,
        question: question_mapper::to_dto(game_progress.question_content),
    };
    let answer = answer_to_entity(answer);
    game_service.save_game_answer(&answer).await;
    debug!("game_progress_answer ending");
}

/// POST request to register new player
#[post("/game/<id>/users/<user>", format = "json")]
pub async fn game_register_user(id: String, user: String) {
    debug!("game_register_user started");
    let game_service = GameService::init().await;
    let mut game = game_service
        .get_game(id)
        .await
        .expect("Failed to get game progress");
    if game.users.contains(&user) == false {
        game.users.push(user);
        game_service.save_users_in_game(&game).await;
    }
    debug!("game_register_user ending");
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

/// GET request to get a game answers.
/// Returns the game result.
/// Returns an error if the game does not exist.
#[get("/game/<id>/answers", format = "json")]
pub async fn get_game_answers(id: String) -> Result<Json<Vec<GameAnswerDto>>, Status> {
    debug!("get_game_result resource started");
    let game_service = GameService::init().await;
    let game_progress = game_service.get_game_progress(id.clone()).await;
    match game_progress {
        Ok(game_progress) => {
            if game_progress.current_question < game_progress.question_number {
                return Err(Status::Locked);
            }
        }
        Err(error) => return Err(process_service_error(error)),
    }
    let game_answers = game_service.get_game_result(id.clone()).await;
    let result = match game_answers {
        Ok(answers_fetched) => {
            let game_output: Vec<GameAnswerDto> = answers_fetched
                .iter()
                .map(|answer| game_mapper::entity_to_answer(answer.clone()))
                .collect();
            Ok(Json(game_output))
        }
        Err(err) => Err(process_service_error(err)),
    };
    debug!("get_game_result resource ending");
    result
}

fn resolve_question_pool(game: &Game) -> Vec<QuestionDto> {
    let mut questions = vec![];
    let topics = game.topics.clone();
    if topics.contains(&"Java".to_string()) {
        questions.append(&mut questions_java());
    }
    if topics.contains(&"Rust".to_string()) {
        questions.append(&mut questions_rust());
    }
    if topics.contains(&"Kotlin".to_string()) {
        questions.append(&mut questions_kotlin());
    }
    questions
}

async fn start_new_game(id: String) {
    info!("Starting the game");
    let game_service = GameService::init().await;
    let game = game_service.get_game(id.clone()).await;
    if let Ok(game) = game {
        let questions = resolve_question_pool(&game);
        let random_index = rand::thread_rng().gen_range(0..questions.len());
        let question = questions.get(random_index).unwrap().clone();
        let mut game_proress_dto = GameProgressDto {
            game_id: game.id.unwrap().to_string(),
            current_question: 0,
            question_number: game.question_number,
            question_content: question.clone(),
        };
        let game_progress_entity = progress_to_entity(game_proress_dto.clone());
        game_service.save_game_progress(&game_progress_entity).await;
        let mut interval = time::interval(Duration::from_secs(1));
        for _ in 0..game_proress_dto.question_number {
            for _ in 0..QUESTION_SECONDS {
                interval.tick().await;
                game_proress_dto.question_content.remaining_time -= 1;
                let game_progress_entity = progress_to_entity(game_proress_dto.clone());
                game_service
                    .replace_game_progress(&game_progress_entity)
                    .await;
            }
            info!("Next question");
            game_proress_dto.current_question += 1;
            let random_index = rand::thread_rng().gen_range(0..questions.len());
            game_proress_dto.question_content =
                questions.get(random_index).unwrap().clone().clone();
            let game_progress_entity = progress_to_entity(game_proress_dto.clone());
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
    use crate::resource::game_resource::{
        create_game, game_progress_answer, get_game, get_game_answers, get_games, patch_game,
        start_new_game,
    };
    use log::info;
    use rocket::async_test;
    use rocket::http::Status;
    use rocket::serde::json::Json;
    use rocket::tokio::task;
    use serial_test::serial;
    use std::env;
    use std::thread::sleep;
    use std::time::Duration;
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
            users: vec![],
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
            users: vec![],
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
            users: vec![],
        };
        let new_game_private = GameDto {
            id: None,
            topics: vec!["Java".to_string()],
            question_number: 10,
            is_private: true,
            is_started: false,
            creator: Some("bob".to_string()),
            users: vec![],
        };
        let new_game_started = GameDto {
            id: None,
            topics: vec!["Java".to_string()],
            question_number: 10,
            is_private: false,
            is_started: true,
            creator: Some("bob".to_string()),
            users: vec![],
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
            users: vec![],
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
            users: vec![],
        };
        let _ = patch_game(game_id.clone()).await;

        //Verify that the game was inserted in the DB
        let game_db = get_game(game_id).await.unwrap().into_inner();
        assert_eq!(game_db.is_started, true);
    }

    #[async_test]
    #[serial]
    async fn post_game_progress_answer_should_replace_existing_answer() {
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
            is_started: true,
            creator: Some("bob".to_string()),
            users: vec![],
        };
        let game = create_game(Json(new_game)).await.unwrap().into_inner();
        let id_clone = game.id.clone().unwrap();
        task::spawn(async move { start_new_game(id_clone.clone()).await });
        sleep(Duration::from_millis(100));
        game_progress_answer(game.id.clone().unwrap(), 1, game.creator.clone().unwrap()).await;
        let answers = get_game_answers(game.id.clone().unwrap())
            .await
            .unwrap()
            .into_inner();
        assert_eq!(answers.len(), 1);
        game_progress_answer(game.id.clone().unwrap(), 1, game.creator.unwrap()).await;
        let answers = get_game_answers(game.id.unwrap())
            .await
            .unwrap()
            .into_inner();
        assert_eq!(answers.len(), 1);
    }
}
