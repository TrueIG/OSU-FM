use reqwest::Client;
use std::{collections::HashMap, rc::Rc};

use super::models::{Beatmap, OAuthTokenResponse};

pub struct OsuService {
    client: Rc<Client>,
    client_id: String,
    client_secret: String,
    user_id: String,
}

impl OsuService {
    pub fn new(
        client: Rc<Client>,
        client_id: String,
        client_secret: String,
        user_id: String,
    ) -> Self {
        Self {
            client,
            client_id,
            client_secret,
            user_id,
        }
    }

    pub async fn get_auth_token(&self) -> Result<OAuthTokenResponse, reqwest::Error> {
        let params = self.get_params();

        const AUTH_URL: &str = "https://osu.ppy.sh/oauth/token";

        self.client
            .post(AUTH_URL)
            .form(&params)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_beatmap(&self, token: &str) -> Result<Option<Beatmap>, reqwest::Error> {
        let beatmap_url = format!(
            "https://osu.ppy.sh/api/v2/users/{}/scores/recent?limit=1",
            self.user_id
        );

        let res: Vec<Beatmap> = self
            .client
            .get(beatmap_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .json()
            .await?;

        Ok(res.into_iter().next())
    }

    fn get_params(&self) -> HashMap<String, String> {
        HashMap::from([
            ("client_id".to_string(), self.client_id.to_string()),
            ("client_secret".to_string(), self.client_secret.to_string()),
            ("grant_type".to_string(), "client_credentials".to_string()),
            ("scope".to_string(), "public".to_string()),
        ])
    }
}
