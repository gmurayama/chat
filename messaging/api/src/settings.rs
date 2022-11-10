use eyre::Context;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct Jaeger {
    pub host: String,
    pub port: u32,
}

#[derive(serde::Deserialize, Clone)]
pub struct Database {
    pub url: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_size: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timeout: u64,
}

#[derive(serde::Deserialize, Clone)]
pub struct Application {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub environment: Environment,
    pub service_name: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub app: Application,
    pub database: Database,
    pub jaeger: Jaeger,
}

pub fn get_config() -> eyre::Result<Settings> {
    let settings = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("_"),
        )
        // App default settings
        .set_default("app.host", "localhost")?
        .set_default("app.port", 8080)?
        .set_default("app.environment", Environment::Development.as_str())?
        .set_default("app.service_name", "messaging")?
        // Database default settings
        .set_default(
            "database.url",
            "postgres://user:password@localhost:8001/messaging?sslmode=disable",
        )?
        .set_default("database.max_size", 10)?
        .set_default("database.timeout", 10)?
        // Jaeger default settings
        .set_default("jaeger.host", "127.0.0.1")?
        .set_default("jaeger.port", 6831)?
        .build()
        .wrap_err("error loading configuration from env variables")?;

    settings
        .try_deserialize::<Settings>()
        .wrap_err("error deserializing settings")
}

#[derive(serde::Deserialize, Clone, PartialEq)]
pub enum Environment {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "production")]
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
