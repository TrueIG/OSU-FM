use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::Write};

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastfmConfig {
    pub sk: Box<str>,
}

impl Vars {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(Self {
            osu_client_secret: get_var("OSU_CLIENT_SECRET")?.into(),
            osu_user_id: get_var("OSU_USER_ID")?.into(),
            osu_client_id: get_var("OSU_CLIENT_ID")?.into(),
            lastfm_api_key: get_var("LASTFM_API_KEY")?.into(),
            lastfm_shared_secret: get_var("LASTFM_SHARED_SECRET")?.into(),
        })
    }
}

fn get_var(name: &str) -> Result<String, env::VarError> {
    env::var(name).map_err(|e| {
        eprintln!("Error to get {name}: {e}");
        e
    })
}

pub fn create_config(sk: Box<str>, token: Box<str>) -> Result<Config, Box<dyn std::error::Error>> {
    let config = Config {
        lastfm: LastfmConfig { sk },
        osu: OsuConfig {
            token,
            last_track: None,
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
