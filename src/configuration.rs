use config::{Config, File, FileFormat};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application_port: u16,
    pub application_host: String,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn get_url_to_bind(&self) -> String {
        format!("{}:{}", self.application_host, self.application_port)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut mode = "debug";
    if !cfg!(debug_assertions) {
        mode = "release"
    }
    let build_mode_configuration = format!("configuration.{}.json", mode);

    let builder = Config::builder()
        .add_source(File::new("configuration.json", FileFormat::Json))
        .add_source(File::new(&build_mode_configuration, FileFormat::Json).required(false))
        .add_source(File::new("configuration.local.json", FileFormat::Json).required(false));
    let config = builder.build()?;

    config.try_deserialize()
}
