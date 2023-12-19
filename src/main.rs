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
