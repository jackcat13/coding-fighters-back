mod dto;
mod fairing;
mod mapper;
mod model;
mod repository;
mod resource;
mod service;

use crate::fairing::cors::Cors;
use crate::resource::game_resource::get_games;
use resource::game_resource::create_game;
use rocket::{get, routes, Build, Rocket};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = build_rocket().launch().await?;
    Ok(())
}

fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![create_game])
        .mount("/", routes![get_games])
        .attach(Cors)
}

#[cfg(test)]
mod test {
    use super::{build_rocket};
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::uri;
    use crate::dto::game_dto::GameDto;
    use crate::resource::game_resource::rocket_uri_macro_get_games;

    #[test]
    fn games_should_return_games() {
        let client = Client::tracked(build_rocket()).expect("valid rocket instance");
        let response = client.get(uri!(get_games)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let games: Vec<GameDto> = response.into_json().expect("Valid parsing of fetched games");
        assert_eq!(games, vec![GameDto{
            id: None,
            topics: vec![],
            question_number: "".to_string(),
            is_private: false,
        }]);
    }
}
