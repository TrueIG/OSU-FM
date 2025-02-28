use dotenv::dotenv;
use std::{env, fs};

pub fn create_file(file_name: &str, content: &str) {
    let _ = fs::write(file_name, content);
}

pub fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_name)
}

pub struct Config {
    pub osu_client_secret: String,
    pub osu_user_id: String,
    pub osu_client_id: String,
    pub lastfm_api_key: String,
    pub lastfm_shared_secret: String,
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
