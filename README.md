# OSU-FM
![OSU-FM](https://i.imgur.com/ZFBHiW7.png)

## Build from source
### Install Bun
Bun is a develop, test, run, and bundle JavaScript & TypeScript projects—all with Bun [bun.sh](https://bun.sh).

### Install Dependencies
```shell
cd OSU-FM
bun install
```

## Environment variables
You will need a [LASTFM](https://www.last.fm/api/account/create) and [OSU](https://osu.ppy.sh/home/account/edit#oauth) API key. They are necessary for the program to work.

And you need a [Osu user id](https://osu.ppy.sh/community/forums/topics/1306439?n=1)

After obtaining them, you can copy or rename ``.env.example`` file to ``.env`` and put them ``OSU_CLIENT_ID``, ``OSU_CLIENT_SECRET``, ``OSU_USER_ID``, ``LASTFM_API_KEY``, ``LASTF_SECRET``.

Like this:
```.env
OSU_CLIENT_ID=your_osu_client_id
OSU_CLIENT_SECRET=your_osu_client_secret
OSU_USER_ID=your_osu_user_id
LASTFM_API_KEY=your_lastfm_api_key
LASTFM_SECRET=your_lastfm_secret
```

## Build
### Built the project
Build the project by using this command:
```
bun run build
```

## Running
First you need to get the last fm authentication key by running this file:
```
dist/auth-lastfm
```

And then run the program:
```
dist/osu-fm
```
