use reqwest::Client;
use std::{collections::HashMap, rc::Rc};

use super::models::{Beatmap, OAuthTokenResponse};

pub struct OsuService {
    client: Rc<Client>,
    client_id: Box<str>,
    client_secret: Box<str>,
    user_id: Box<str>,
}

impl OsuService {
    pub fn new(
        client: Rc<Client>,
        client_id: Box<str>,
        client_secret: Box<str>,
        user_id: Box<str>,
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

    fn get_params(&self) -> HashMap<Box<str>, Box<str>> {
        HashMap::from([
            ("client_id".into(), self.client_id.clone()),
            ("client_secret".into(), self.client_secret.clone()),
            ("grant_type".into(), "client_credentials".into()),
            ("scope".into(), "public".into()),
        ])
    }
}
