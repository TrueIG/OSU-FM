use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthTokenResponse {
    pub token_type: String,
    pub expires_in: i32,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeatmapSet {
    pub artist_unicode: String,
    pub title_unicode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beatmap {
    pub beatmapset: BeatmapSet,
    pub created_at: String,
    pub id: i64,
}
