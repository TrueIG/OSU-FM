use std::{path::Path, time::Duration};

use api::{lastfm::lastfm::LastFmService, osu::osu::OsuService};

use osu_fm::{create_file, read_file, Config};
use tokio::time::sleep;

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let mut lastfm_service = LastFmService::new(config.lastfm_api_key, config.lastfm_shared_secret);
    let osu_service = OsuService::new(
        config.osu_client_id,
        config.osu_client_secret,
        config.osu_user_id,
    );

    if !Path::new("osu-token.txt").exists() || !Path::new("lastfm-sk.txt").exists() {
        first_entry(&osu_service, &mut lastfm_service).await;
    }

    let _ = monitor_beatmap_updates(&osu_service, &mut lastfm_service).await;

    Ok(())
}

async fn monitor_beatmap_updates(
    osu_service: &OsuService,
    lastfm_service: &mut LastFmService,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = read_file("osu-token.txt")?;
    let sk = read_file("lastfm-sk.txt")?;

    let mut last_track = read_file("last_track.txt").unwrap_or_default();

    loop {
        match osu_service.get_beatmap(&token).await {
            Ok(Some(beatmap)) if last_track != beatmap.id.to_string() => {
                last_track = beatmap.id.to_string();
                create_file("last_track.txt", &last_track);
                let result = lastfm_service
                    .scrobbe(
                        &beatmap.beatmapset.artist_unicode,
                        &beatmap.beatmapset.title_unicode,
                        &sk,
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

async fn first_entry(osu_service: &OsuService, lastfm_service: &mut LastFmService) {
    let lastfm_sk = lastfm_service.init().await;
    create_file("lastfm-sk.txt", &lastfm_sk.unwrap());

    let osu_token = osu_service.get_auth_token().await;
    create_file("osu-token.txt", &osu_token.unwrap().access_token);
}
