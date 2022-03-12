mod config;
mod discovery_service;
mod house_service;
mod http_handlers;

use config::{Config, MongoConfig};
use house_service::HousesService;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use warp::Filter;

use crate::{discovery_service::DiscoveryService, http_handlers::log_req};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = Config::try_from_env().unwrap();

    let db = connect_to_mongo(&config.mongodb).await.unwrap();

    let collection = db.collection(&config.mongodb.house_collection);
    let houses_service = HousesService::new(collection);
    let houses_service = warp::any().map(move || houses_service.clone());

    let discovery_service = DiscoveryService::new();
    let discovery_service = warp::any().map(move || discovery_service.clone());

    let insert_house = warp::path!("api" / "houses")
        .and(log_req())
        .and(warp::post())
        .and(warp::body::json())
        .and(houses_service.clone())
        .and_then(http_handlers::insert_house);

    let remove_house = warp::path!("api" / "houses" / String)
        .and(log_req())
        .and(warp::delete())
        .and(houses_service.clone())
        .and_then(http_handlers::remove_house);

    let get_houses = warp::path!("api" / "houses")
        .and(warp::get())
        .and(log_req())
        .and(houses_service.clone())
        .and_then(http_handlers::get_houses);

    let discover = warp::path!("api" / "discover")
        .and(warp::get())
        .and(log_req())
        .and(discovery_service.clone())
        .and(warp::query::<http_handlers::DiscoverQueryParameter>())
        .and_then(http_handlers::discover);

    let static_files = warp::get().and(warp::fs::dir(config.static_directory));

    let router = insert_house
        .or(get_houses)
        .or(remove_house)
        .or(discover)
        .or(static_files)
        .recover(http_handlers::handle_rejection);

    let server = warp::serve(router);

    info!("starting server....");
    server.bind(([0, 0, 0, 0], config.http_port)).await;
}

async fn connect_to_mongo(
    config: &MongoConfig,
) -> Result<mongodb::Database, mongodb::error::Error> {
    info!("connecting to mongo...");

    let mut client_options =
        ClientOptions::parse_with_resolver_config(config.url.clone(), ResolverConfig::cloudflare())
            .await?;

    let mut credential: mongodb::options::Credential = Default::default();
    credential.username = Some(config.username.clone());
    credential.password = Some(config.password.clone());

    client_options.credential = Some(credential);
    client_options.app_name = Some("preference-be".to_string());

    let client = Client::with_options(client_options)?;
    info!("connected");

    let db = client.database(&config.database.clone());

    Ok(db)
}
