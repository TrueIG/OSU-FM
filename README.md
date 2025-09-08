<br>
<div align="center">
<img src="https://i.imgur.com/ISeFIxz.png" width="144"/>
  <h1 align="center">OSU-FM</h1>
  <p align="center">
    <strong>Scrobble your songs from osu! beatmaps.</strong>
  </p>
</div>

## About 
**Osu-fm** is an integration of the **osu!** and **Last.fm** APIs, a simple service that allows you to **scrobble osu! beatmaps**.

The project is a recreation of the old [Osu-fm](https://github.com/TrueIG/OSU-FM-TS), originally made in TypeScript, now rewritten in **Rust**.

## Features
- **Beatmap Blacklist:** Blacklist certain beatmaps from scrobbling.
- **Beatmap config:** Edit name on a scrobble from a certain beatmap.
- **Beatmap title clean:** Remove keywords like "nightcore", "cut ver.", "speed up" etc.

## Installation

### Install Rust 
A language empowering everyone to build reliable and efficient software. download and install it from [rust-lang.org](https://www.rust-lang.org).

### Environment variables
You will need a [LASTFM](https://www.last.fm/api/account/create) and [OSU](https://osu.ppy.sh/home/account/edit#oauth) API key. They are necessary for the program to work.

And you need a [Osu user id](https://osu.ppy.sh/community/forums/topics/1306439?n=1).

After obtaining them, you can copy or rename ``.env.example`` file to ``.env`` and put them ``OSU_CLIENT_ID``, ``OSU_CLIENT_SECRET``, ``OSU_USER_ID``, ``LASTFM_API_KEY``, ``LASTFM_SHARED_SECRET``.

```.env
OSU_CLIENT_ID=<your_osu_client>
OSU_CLIENT_SECRET=<your_osu_client_secret>
OSU_USER_ID=<your_osu_user_id>
LASTFM_API_KEY=<your_lastfm_api_key>
LASTFM_SHARED_SECRET=<your_lastm_shared_secret>
```

### Build and run the project
Build the project by using this command:
```zsh
cargo build --release                                          
```
The binary will be generated inside ``target/release/osu_fm``. Just run the binary file that the program is running.
