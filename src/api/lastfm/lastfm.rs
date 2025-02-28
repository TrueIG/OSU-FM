use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use md5::{Digest, Md5};
use tokio::time::sleep;

use super::models::{SessionResponse, Token};

use reqwest::Client;
use webbrowser::{self, open};

pub struct LastFmService {
    client: Client,
    api_key: String,
    shared_secret: String,
    token: Option<String>,
}

impl LastFmService {
    pub fn new(api_key: String, shared_secret: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            shared_secret,
            token: None,
        }
    }

    fn generate_signature(&self, params: &mut HashMap<String, String>) {
        let mut sorted_keys: Vec<String> = params.keys().cloned().collect();
        sorted_keys.sort();

        let mut string_to_hash = String::new();

        for key in sorted_keys {
            if key == "format" {
                continue;
            }
            string_to_hash.push_str(&key);
            if let Some(value) = params.get(&key) {
                string_to_hash.push_str(value);
            }
        }

        string_to_hash.push_str(&self.shared_secret);

        let mut hasher = Md5::new();
        hasher.update(string_to_hash);

        let api_sig = format!("{:x}", hasher.finalize());
        params.insert("api_sig".to_string(), api_sig);
    }

    async fn get_session(&self) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.token.clone();

        let mut params = HashMap::from([
            ("method".to_string(), "auth.getSession".to_string()),
            ("token".to_string(), token.unwrap()),
            ("api_key".to_string(), self.api_key.clone()),
            ("format".to_string(), "json".to_string()),
        ]);

        self.generate_signature(&mut params);

        loop {
            let response = self
                .client
                .post("https://ws.audioscrobbler.com/2.0/")
                .form(&params)
                .send()
                .await?
                .json()
                .await?;

            println!("result {:#?}", response);

            match response {
                SessionResponse::Success { session } => {
                    return Ok(session.key);
                }
                SessionResponse::Error { error, message } => {
                    eprintln!("Error {}: {}", error, message);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }

    async fn get_token(&mut self) -> Result<(), reqwest::Error> {
        let url = format!(
            "https://ws.audioscrobbler.com/2.0/?method=auth.gettoken&api_key={}&format=json",
            self.api_key
        );

        let token = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Token>()
            .await?
            .token;

        self.token = Some(token);

        Ok(())
    }

    async fn get_user_authorization(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = match &self.token {
            Some(token) => format!(
                "http://www.last.fm/api/auth/?api_key={}&token={}",
                self.api_key, token
            ),
            None => {
                eprintln!("Erro: Token not defined.");
                return Ok(());
            }
        };

        open(&url)?;
        Ok(())
    }

    pub async fn scrobbe(&self, artist: &str, track: &str, sk: &str) -> Result<(), reqwest::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut params = HashMap::from([
            ("method".to_string(), "track.scrobble".to_string()),
            ("artist".to_string(), artist.to_string()),
            ("track".to_string(), track.to_string()),
            ("api_key".to_string(), self.api_key.clone()),
            ("sk".to_string(), sk.to_string()),
            ("timestamp".to_string(), timestamp.to_string()),
        ]);

        self.generate_signature(&mut params);

        let url = "https://ws.audioscrobbler.com/2.0/";

        let response = self.client.post(url).form(&params).send().await?;

        println!("{:#?}", response);

        Ok(())
    }

    pub async fn init(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let _ = self.get_token().await;
        let _ = self.get_user_authorization().await;
        let response = self.get_session().await?;
        Ok(response)
    }
}
