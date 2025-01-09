use config::ConfigError;
use config::File;
use config::FileFormat;
use secrecy::ExposeSecret;
use secrecy::SecretString;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
    pub database: DatabaseConfiguration,
}

#[derive(serde::Deserialize)]
pub struct ApplicationConfiguration {
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfiguration {
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
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database,
        ))
    }
}

pub fn get_configuration(file: &str) -> Result<Configuration, ConfigError> {
    let configuration = config::Config::builder()
        .add_source(File::new(file, FileFormat::Yaml))
        .build()?;

    configuration.try_deserialize::<Configuration>()
}
