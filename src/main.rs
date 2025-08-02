use std::{
    fs::{self},
    rc::Rc,
    time::Duration,
};

use api::{lastfm::lastfm::LastFmService, osu::osu::OsuService};

use osu_fm::{create_config, write_config, Config as CF, Vars};
use reqwest::Client;
use simplelog::*;
use spinoff::{spinners, Color, Spinner};
use std::fs::File;
use tokio::time::sleep;

use crate::api::osu::models::OAuthTokenResponse;

mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::options()
                .append(true)
                .create(true)
                .open("log.txt")
                .unwrap(),
        ),
    ])
    .unwrap();

    log::info!("Starting OSU-FM");

    let mut sp =
        Spinner::new(spinners::Dots, "Reading environment variables...", None);
    let vars = Vars::from_env()?;
    sp.success("Enviroment variables success!");

    let client = Rc::new(Client::new());

    let mut sp = Spinner::new(spinners::Dots, "Creating services...", None);
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

    sp.success("Services success!");

    let osu_token: String = match osu_service.get_auth_token().await.unwrap() {
        OAuthTokenResponse::Success(token) => token.access_token,
        OAuthTokenResponse::Error(err) => {
            log::error!("{:?}", err);
            panic!(
                "\nError: {}\nDescription: {}\nNote: Check if `OSU_CLIENT_ID` and `OSU_CLIENT_SECRET` are correctly set in the environment.",
                err.error, err.error_description
            );
        }
    };

    let mut configs: CF = if let Ok(content) = fs::read_to_string("config.json")
    {
        serde_json::from_str(&content)?
    } else {
        let mut sp = Spinner::new(
            spinners::Dots,
            "Waiting user authorization...",
            Color::Blue,
        );
        let config = create_config(
            lastfm_service.init().await.unwrap(),
            osu_token.clone().into(),
        )
        .unwrap();
        sp.success("User authorization success!");
        config
    };

    configs.osu.token = osu_token.into();

    let _ = write_config(&serde_json::to_string_pretty(&configs)?);
    let _ = monitor_beatmap_updates(
        &osu_service,
        &mut lastfm_service,
        &mut configs,
    )
    .await;

    Ok(())
}

async fn monitor_beatmap_updates(
    osu_service: &OsuService,
    lastfm_service: &mut LastFmService,
    config: &mut CF,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut spinner = Spinner::new(
            spinners::Arrow3,
            "Waiting for new beatmaps...",
            Color::Blue,
        );
        match osu_service.get_beatmap(&config.osu.token).await {
            Ok(Some(beatmap)) if config.osu.last_track != Some(beatmap.id) => {
                config.osu.last_track = Some(beatmap.id);
                let _ = write_config(&serde_json::to_string_pretty(&config)?);
                let _result = lastfm_service
                    .scrobbe(
                        &beatmap.beatmapset.artist_unicode,
                        &beatmap.beatmapset.title_unicode,
                        &config.lastfm.sk,
                    )
                    .await;

                spinner.info("New Scrobbe!");
                log::info!(
                    " Artist: {}\n󰎇 Title: {}",
                    &beatmap.beatmapset.artist_unicode,
                    &beatmap.beatmapset.title_unicode,
                )
            }
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        }

        sleep(Duration::from_secs(10)).await;
    }
}
