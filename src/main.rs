use std::{
    fs::{self},
    rc::Rc,
    time::Duration,
};

use api::{lastfm::lastfm::LastFmService, osu::osu::OsuService};

use osu_fm::{create_config, write_config, Config, Vars};
use reqwest::Client;
use tokio::time::sleep;

mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vars = Vars::from_env()?;

    let client = Rc::new(Client::new());

    let mut lastfm_service = LastFmService::new(
        client.clone(),
        vars.lastfm_api_key,
        vars.lastfm_shared_secret,
    );

    let osu_service = OsuService::new(
        client.clone(),
        vars.osu_client_id,
        vars.osu_client_secret,
        vars.osu_user_id,
    );

    let config = "config.json";

    let mut configs: Config = if let Ok(content) = fs::read_to_string(config) {
        serde_json::from_str(&content)?
    } else {
        create_config(
            lastfm_service.init().await.unwrap().into(),
            osu_service
                .get_auth_token()
                .await
                .unwrap()
                .access_token
                .into(),
        )
        .unwrap()
    };

    let _ = write_config(&serde_json::to_string_pretty(&configs)?);
    let _ = monitor_beatmap_updates(&osu_service, &mut lastfm_service, &mut configs).await;

    Ok(())
}

async fn monitor_beatmap_updates(
    osu_service: &OsuService,
    lastfm_service: &mut LastFmService,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match osu_service.get_beatmap(&config.osu.token).await {
            Ok(Some(beatmap)) if config.osu.last_track != Some(beatmap.id) => {
                config.osu.last_track = Some(beatmap.id);
                let _ = write_config(&serde_json::to_string_pretty(&config)?);
                let result = lastfm_service
                    .scrobbe(
                        &beatmap.beatmapset.artist_unicode,
                        &beatmap.beatmapset.title_unicode,
                        &config.lastfm.sk,
                    )
                    .await;

                println!("{:#?}", result);
            }
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        }

        sleep(Duration::from_secs(10)).await;
    }
}
