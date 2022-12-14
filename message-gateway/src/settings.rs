use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct WebsocketConnection {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timeout: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub interval: u64,
}

#[derive(serde::Deserialize, Clone)]
pub struct Application {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub environment: Environment,
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub app: Application,
    pub ws_conn: WebsocketConnection,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("_"),
        )
        .set_default("app.host", "localhost")?
        .set_default("app.port", 8080)?
        .set_default("app.environment", Environment::Development.as_str())?
        .set_default("ws_conn.timeout", 10)?
        .set_default("ws_conn.interval", 5)?
        .build()?;

    settings.try_deserialize::<Settings>()
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
