use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OAuthTokenResponse {
    Success(OAuthTokenSuccess),
    Error(OAuthTokenError),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Ruleset {
    Fruits,
    Mania,
    Osu,
    Taiko,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthTokenError {
    pub error: String,
    pub error_description: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthTokenSuccess {
    pub token_type: String,
    pub expires_in: i32,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapSet {
    pub artist_unicode: String,
    pub title_unicode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Beatmap {
    pub beatmapset: BeatmapSet,
    pub created_at: String,
    pub id: i64,
}
