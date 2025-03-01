use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::Write};

pub struct Config {
    pub osu_client_secret: String,
    pub osu_user_id: String,
    pub osu_client_id: String,
    pub lastfm_api_key: String,
    pub lastfm_shared_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infos {
    pub last_track: Option<i64>,
    pub sk: Box<str>,
    pub token: Box<str>,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(Self {
            osu_client_secret: env::var("OSU_CLIENT_SECRET")?,
            osu_user_id: env::var("OSU_USER_ID")?,
            osu_client_id: env::var("OSU_CLIENT_ID")?,
            lastfm_api_key: env::var("LASTFM_API_KEY")?,
            lastfm_shared_secret: env::var("LASTFM_SHARED_SECRET")?,
        })
    }
}

pub fn create_info(sk: Box<str>, token: Box<str>) -> Result<Infos, Box<dyn std::error::Error>> {
    let infos = Infos {
        sk,
        token,
        last_track: None,
    };
    let data = serde_json::to_string_pretty(&infos)?;

    let _ = write_info(&data);

    Ok(infos)
}

pub fn write_info(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("infos.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
