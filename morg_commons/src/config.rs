use std::io::{Read, ErrorKind};
use std::fs::{OpenOptions};
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct ReturnedConfig {
    pub address: Option<String>,
    pub database: Option<DatabaseSettings>, 
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub address: String,
    pub password: String,
    pub username: String,
    pub database: String,
}

impl Default for ReturnedConfig {
    fn default() -> Self {
        Self {
            database: None,
            address: Some("127.0.0.1:6880".to_owned()),
        }
    }
}

pub type Result = std::result::Result<ReturnedConfig, Box<dyn std::error::Error>>;

pub async fn fetch(path: &Path) -> Result {
    match OpenOptions::new()
        .read(true)
        .open(path)
    {
        Ok(mut file) => {
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("cannot read contents from string.");

            match serde_yaml::from_str::<ReturnedConfig>(&contents) {
                Ok(returned) => Ok(pass_values_into_env(returned).await),
                Err(error) => Err(Box::new(error))
            }
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(
                pass_values_into_env(ReturnedConfig::default()).await
            ),
            _ => Err(Box::new(error))
        }
    }
}

async fn pass_values_into_env(values: ReturnedConfig) -> ReturnedConfig {
    let db_values = values.clone().database;

    ReturnedConfig {
        address: Some(asynv::get("MORG_ADDR").await
            .unwrap_or_else(|_| values.address.unwrap_or_default())),
        database: if db_values.is_none() {
            if asynv::get("MORG_DB_ADDR").await.is_err() {
                panic!("An database config is not present, please provide settings in the config.")
            } else {
                Some(DatabaseSettings {
                    address: asynv::get("MORG_DB_ADDR").await
                        .unwrap_or_else(|_| String::new()),
                    password: asynv::get("MORG_DB_PASS").await
                        .unwrap_or_else(|_| String::new()),
                    username: asynv::get("MORG_DB_USER").await
                        .unwrap_or_else(|_| String::new()),
                    database: asynv::get("MORG_DB_NAME").await
                        .unwrap_or_else(|_| String::new()),
                })
            }
        } else {
            Some(DatabaseSettings {
                address: asynv::get("MORG_DB_ADDR").await
                    .unwrap_or_else(|_| {
                        db_values.clone().unwrap().address
                    }),
                password: asynv::get("MORG_DB_PASS").await
                    .unwrap_or_else(|_| {
                        db_values.clone().unwrap().password
                    }),
                username: asynv::get("MORG_DB_USER").await
                    .unwrap_or_else(|_| {
                        db_values.clone().unwrap().username
                    }),
                database: asynv::get("MORG_DB_NAME").await
                    .unwrap_or_else(|_| {
                        db_values.clone().unwrap().database
                    })
            })
        },
        ..values
    }
}