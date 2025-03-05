use std::{
    collections::HashMap,
    rc::Rc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use md5::{Digest, Md5};
use tokio::time::sleep;

use super::models::{SessionResponse, Token};

use reqwest::Client;
use webbrowser::{self, open};

pub struct LastFmService {
    client: Rc<Client>,
    api_key: Box<str>,
    shared_secret: Box<str>,
    token: Option<Box<str>>,
}

impl LastFmService {
    pub fn new(client: Rc<Client>, api_key: Box<str>, shared_secret: Box<str>) -> Self {
        Self {
            client,
            api_key,
            shared_secret,
            token: None,
        }
    }

    fn generate_signature(&self, params: &mut HashMap<Box<str>, Box<str>>) {
        let mut sorted_keys: Vec<Box<str>> = params.keys().cloned().collect();
        sorted_keys.sort();

        let mut string_to_hash = String::new();

        for key in sorted_keys {
            if key.as_ref() == "format" {
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
        params.insert("api_sig".into(), api_sig.into());
    }

    async fn get_session(&self) -> Result<Box<str>, Box<dyn std::error::Error>> {
        let token = self.token.clone();

        let mut params: HashMap<Box<str>, Box<str>> = HashMap::from([
            ("method".into(), "auth.getSession".into()),
            ("token".into(), token.unwrap()),
            ("api_key".into(), self.api_key.clone()),
            ("format".into(), "json".into()),
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
                    return Ok(session.key.into());
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

        self.token = Some(token.into());

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

        let mut params: HashMap<Box<str>, Box<str>> = HashMap::from([
            ("method".into(), "track.scrobble".into()),
            ("artist".into(), artist.into()),
            ("track".into(), track.into()),
            ("api_key".into(), self.api_key.clone()),
            ("sk".into(), sk.into()),
            ("timestamp".into(), timestamp.to_string().into()),
        ]);

        self.generate_signature(&mut params);

        let url = "https://ws.audioscrobbler.com/2.0/";

        let response = self.client.post(url).form(&params).send().await?;

        println!("{:#?}", response);

        Ok(())
    }

    pub async fn init(&mut self) -> Result<Box<str>, Box<dyn std::error::Error>> {
        let _ = self.get_token().await;
        let _ = self.get_user_authorization().await;
        let response = self.get_session().await?;
        Ok(response)
    }
}
