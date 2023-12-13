mod dto;
mod mapper;
mod model;
mod repository;
mod resource;
mod service;

use resource::game_resource::create_game;
use rocket::{get, routes, Build, Rocket};

#[get("/")]
fn index() -> &'static str {
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
        .mount("/game", routes![create_game])
}
