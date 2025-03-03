use std::{
    fs::{self},
    rc::Rc,
    time::Duration,
};

use api::{lastfm::lastfm::LastFmService, osu::osu::OsuService};

use osu_fm::{create_info, write_info, Config, Infos};
use reqwest::Client;
use tokio::time::sleep;

mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let client = Rc::new(Client::new());

    let mut lastfm_service = LastFmService::new(
        client.clone(),
        config.lastfm_api_key,
        config.lastfm_shared_secret,
    );

    let osu_service = OsuService::new(
        client.clone(),
        config.osu_client_id,
        config.osu_client_secret,
        config.osu_user_id,
    );

    let path_json = "infos.json";

    let mut infos: Infos = if let Ok(content) = fs::read_to_string(path_json) {
        serde_json::from_str(&content)?
    } else {
        create_info(
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

    let _ = write_info(&serde_json::to_string_pretty(&infos)?);
    let _ = monitor_beatmap_updates(&osu_service, &mut lastfm_service, &mut infos).await;

    Ok(())
}

async fn monitor_beatmap_updates(
    osu_service: &OsuService,
    lastfm_service: &mut LastFmService,
    infos: &mut Infos,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match osu_service.get_beatmap(&infos.token).await {
            Ok(Some(beatmap)) if infos.last_track != Some(beatmap.id) => {
                infos.last_track = Some(beatmap.id);
                let _ = write_info(&serde_json::to_string_pretty(&infos)?);
                let result = lastfm_service
                    .scrobbe(
                        &beatmap.beatmapset.artist_unicode,
                        &beatmap.beatmapset.title_unicode,
                        &infos.sk,
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
