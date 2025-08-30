use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::Write};
use thiserror::Error;

#[derive(Debug)]
pub struct Vars {
    pub osu_client_secret: Box<str>,
    pub osu_user_id: Box<str>,
    pub osu_client_id: Box<str>,
    pub lastfm_api_key: Box<str>,
    pub lastfm_shared_secret: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub osu: OsuConfig,
    pub lastfm: LastfmConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsuConfig {
    pub last_track: Option<i64>,

    pub token: Box<str>,
    pub regex: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastfmConfig {
    pub sk: Box<str>,
}

#[derive(Debug, Error)]
pub enum EnvVarError {
    #[error("Failed to get env var '{0}': {1}")]
    MissingVar(String, #[source] env::VarError),
}

impl EnvVarError {
    pub fn name(&self) -> &str {
        match self {
            EnvVarError::MissingVar(name, _) => name,
        }
    }
}

impl Vars {
    pub fn from_env() -> Result<Self, EnvVarError> {
        dotenv::dotenv().ok();

        Ok(Self {
            osu_client_secret: get_var("OSU_CLIENT_SECRET")?.into(),
            osu_user_id: get_var("OSU_USER_ID")?.into(),
            osu_client_id: get_var("OSU_CLIENT_ID")?.into(),
            lastfm_api_key: get_var("LASTFM_API_KEY")?.into(),
            lastfm_shared_secret: get_var("LASTFM_SHARED_SECRET")?.into(),
        })
    }
}

fn get_var(name: &str) -> Result<String, EnvVarError> {
    env::var(name).map_err(|e| EnvVarError::MissingVar(name.to_string(), e))
}

pub fn create_config(
    sk: Box<str>,
    token: Box<str>,
) -> Result<Config, Box<dyn std::error::Error>> {
    let config = Config {
        lastfm: LastfmConfig { sk },
        osu: OsuConfig {
            token,
            last_track: None,
            regex: Vec::new(),
        },
    };
    let data = serde_json::to_string_pretty(&config)?;

    let _ = write_config(&data);

    Ok(config)
}

pub fn write_config(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("config.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
