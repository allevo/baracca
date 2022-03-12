use envconfig::{Envconfig, Error};

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "STATIC_DIRECTORY")]
    pub static_directory: String,
    #[envconfig(nested = true)]
    pub mongodb: MongoConfig,
    #[envconfig(from = "PORT")]
    pub http_port: u16,
}

impl Config {
    pub(crate) fn try_from_env() -> Result<Self, Error> {
        Config::init_from_env()
    }
}

#[derive(Envconfig)]
pub struct MongoConfig {
    #[envconfig(from = "MONGO_DB_USERNAME")]
    pub username: String,
    #[envconfig(from = "MONGO_DB_PASSWORD")]
    pub password: String,
    #[envconfig(from = "MONGO_DB_URL")]
    pub url: String,
    #[envconfig(from = "MONGO_DB_DATABASE")]
    pub database: String,
    #[envconfig(from = "MONGO_DB_HOUSE_COLLECTION")]
    pub house_collection: String,
}
