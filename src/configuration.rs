
#[derive(serde::Deserialize)]
pub struct Settings {
  pub database: DatabaseSettings,
  pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: String,
  pub database_name: String,
  pub host: String,
  pub port: u16,
}

// read our application settings from a configuration file named configuration.yaml
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let settings = config::Config::builder()
    .add_source(
      config::File::new("configuration.yaml", config::FileFormat::Yaml)
    )
    .build()?;

  settings.try_deserialize::<Settings>()
}