mod dto;
mod errors;
mod fairing;
mod mapper;
mod model;
mod repository;
mod resource;
mod service;

use crate::fairing::cors::Cors;
use crate::fairing::logging::{default_logging_layer, json_logging_layer, LogType};
use crate::fairing::tracing::TracingFairing;
use crate::resource::game_resource::{get_game, get_games};
use resource::game_resource::create_game;
use rocket::config::LogLevel;
use rocket::{get, routes, Build, Rocket};
use std::str::FromStr;
use tracing_log::LogTracer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    init_log_tracer();
    let _rocket = build_rocket().launch().await?;
    Ok(())
}

fn init_log_tracer() {
    LogTracer::init().expect("Unable to setup log tracer!");

    let log_type =
        LogType::from(std::env::var("LOG_TYPE").unwrap_or_else(|_| "formatted".to_string()));
    let log_level = LogLevel::from_str(
        std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "normal".to_string())
            .as_str(),
    )
    .expect("Invalid log level");

    match log_type {
        LogType::Formatted => {
            tracing::subscriber::set_global_default(
                tracing_subscriber::registry()
                    .with(default_logging_layer())
                    .with(filter_layer(log_level)),
            )
            .unwrap();
        }
        LogType::Json => {
            tracing::subscriber::set_global_default(
                tracing_subscriber::registry()
                    .with(json_logging_layer())
                    .with(filter_layer(log_level)),
            )
            .unwrap();
        }
    };
}

pub fn filter_layer(level: LogLevel) -> EnvFilter {
    let filter_str = match level {
        LogLevel::Critical => "warn,hyper=off,rustls=off",
        LogLevel::Normal => "info,hyper=off,rustls=off",
        LogLevel::Debug => "trace",
        LogLevel::Off => "off",
    };
    EnvFilter::try_new(filter_str).expect("filter string must parse")
}

fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![create_game])
        .mount("/", routes![get_games])
        .mount("/", routes![get_game])
        .attach(Cors)
        .attach(TracingFairing)
}

#[cfg(test)]
mod tests {
    use crate::build_rocket;
    use crate::resource::game_resource::rocket_uri_macro_get_games;
    use log::info;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::{async_test, uri};
    use serial_test::serial;
    use std::env;
    use testcontainers::clients::Cli;
    use testcontainers::GenericImage;

    #[async_test]
    #[serial]
    async fn get_games_rest_call_should_return_empty_result() {
        info!("Creating mongo container");
        let docker = Cli::default();
        let container = docker.run(GenericImage::new("mongo", "latest"));
        let port = container.get_host_port_ipv4(27017);
        let uri = format!("mongodb://localhost:{}", port);
        env::set_var("MONGO_URI", uri.clone());
        info!("Mongo container created");

        let client = Client::tracked(build_rocket())
            .await
            .expect("valid rocket instance");
        let response = client.get(uri!(get_games)).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "[]");
    }
}
