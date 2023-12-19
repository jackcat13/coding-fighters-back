mod dto;
mod mapper;
mod model;
mod repository;
mod resource;
mod service;
mod fairing;

use crate::resource::game_resource::get_games;
use resource::game_resource::create_game;
use rocket::{get, routes, Build, Rocket};
use crate::fairing::cors::CORS;

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
        .attach(CORS)
}
