use std::time::Duration;

use config::ConfigError;
use config::File;
use config::FileFormat;
use duration_str::deserialize_duration;
use secrecy::ExposeSecret;
use secrecy::SecretString;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
    pub subscriber: SubscriberConfiguration,
}

#[derive(serde::Deserialize)]
pub struct ApplicationConfiguration {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct SubscriberConfiguration {
    pub database: DatabaseConfiguration,
    pub email: EmailConfiguration,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfiguration {
    pub connection: DatabaseConnectionConfiguration,
    pub pool: DatabasePoolConfiguration,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConnectionConfiguration {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: SecretString,
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.connection.username,
            self.connection.password.expose_secret(),
            self.connection.host,
            self.connection.port,
            self.connection.database,
        ))
    }
}

#[derive(serde::Deserialize)]
pub struct DatabasePoolConfiguration {
    pub min_connections: u32,
    pub max_connections: u32,
    #[serde(deserialize_with = "deserialize_duration")]
    pub acquire_timeout: Duration,
}

#[derive(serde::Deserialize)]
pub struct EmailConfiguration {
    pub server: EmailServerConfiguration,
    pub client: EmailClientConfiguration,
}

#[derive(serde::Deserialize)]
pub struct EmailServerConfiguration {
    pub url: String,
    pub token: SecretString,
}

#[derive(serde::Deserialize)]
pub struct EmailClientConfiguration {
    pub sender: String,
    #[serde(deserialize_with = "deserialize_duration")]
    pub timeout: Duration,
}

pub enum Environment {
    Local,
    Test,
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Test => "test",
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(env: String) -> Result<Self, Self::Error> {
        match env.as_str() {
            "local" => Ok(Environment::Local),
            "test" => Ok(Environment::Test),
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!("{} is not a valid environment", env)),
        }
    }
}

pub fn get_configuration(env: Environment) -> Result<Configuration, ConfigError> {
    let configuration = config::Config::builder()
        .add_source(File::new("configuration/default.yaml", FileFormat::Yaml))
        .add_source(File::new(
            format!("configuration/{}.yaml", env.as_str()).as_str(),
            FileFormat::Yaml,
        ))
        .build()?;

    configuration.try_deserialize::<Configuration>()
}
