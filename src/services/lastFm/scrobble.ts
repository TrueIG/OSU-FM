import { LASTFM_API_KEY, LASTFM_API_URL, log } from "@main/verify-env";
import type { LastFmParamsScrobble, Track } from "@types";
import genereteSignature from "./generete-signature";
import axios from "axios";

export default class Scrobble {
  private sk: string | undefined;

  constructor() {
    this.getSk();
  }

  private async getSk() {
    const file = Bun.file(`${process.cwd()}/token.json`);
    const contents = await file.json();
    this.sk = contents.lfm.session.key;
  }

  public async scrobleTrack(track: Track) {
    const params: LastFmParamsScrobble = {
      method: "track.scrobble",
      api_key: LASTFM_API_KEY,
      artist: track.artist,
      track: track.title,
      timestamp: track.timestamp,
      sk: this.sk!,
    };

    log.info(`New track get: Artist:${params.artist}, Track: ${params.track}`);

    genereteSignature(params);

    try {
      await axios.post(LASTFM_API_URL, null, { params });
    } catch (error) {
      console.error(error);
    }
  }
}
