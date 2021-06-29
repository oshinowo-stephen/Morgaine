use super::stringify;

use std::env::var as load_env;
use std::io::{self, Read};
use std::path::Path;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct LoadConfig {
    pub db_url: String,
    pub address: String,
    pub db_client: String,
    pub db_settings: Option<DatabaseSettings>
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub port: String,
    pub host: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub fn load(conf_path: Option<&str>) -> LoadConfig {
    if let Ok(file_conf) = load_from_file(Path::new(conf_path.unwrap_or_else(|| ""))) {
        LoadConfig {
            address: load_env("MORG_ADDR")
                .unwrap_or_else(|_| file_conf.clone().address),
            db_url: load_env("MORG_DB_URL")
                .unwrap_or_else(|_| file_conf.clone().db_url),
            db_client: load_env("MORG_DB_CLIENT")
                .unwrap_or_else(|_| file_conf.clone().db_client),
            ..Default::default()
        }
    } else {
        LoadConfig::default()
    }
}

fn load_from_file(path: &Path) -> io::Result<LoadConfig> {
    match fs::File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();

            file.read_to_string(&mut contents)?;

            match serde_yaml::from_str::<LoadConfig>(&contents) {
                Ok(config) => Ok(config),
                Err(error) => if let Some(location) = error.location() {
                    error!("Invalid YAML on line: {:?}, loading default config", location);

                    Ok(LoadConfig::default())
                } else {
                    Ok(LoadConfig::default())
                }
            }
        },
        Err(_) => Ok(LoadConfig::default())
    }
}

impl Default for LoadConfig {
    fn default() -> Self {
        Self {
            db_settings: None,
            db_client: stringify!("sqlite"),
            address: stringify!("127.0.0.1:6880"),
            db_url: stringify!("storage/morg_db.sqlite"),
        }
    }
}