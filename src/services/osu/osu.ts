import {
  OSU_CLIENT_ID,
  OSU_CLIENT_SECRET,
  OSU_USER_ID,
} from "@main/verify-env";
import {
  GrantType,
  type OsuAuthToken,
  type OsuParamsAuth,
  type RecentBeatmap,
  type Track,
} from "@types";
import axios from "axios";
import Scrobble from "../lastFm/scrobble";

export default class Osu {
  private accessToken: string | undefined;
  private lastScoreID: number | undefined;

  private async getAcessToken(): Promise<void> {
    const params: OsuParamsAuth = {
      grant_type: GrantType.CREDENTIAL,
      client_id: OSU_CLIENT_ID,
      client_secret: OSU_CLIENT_SECRET,
      scope: "public",
    };

    try {
      const resp: OsuAuthToken = (
        await axios.post("https://osu.ppy.sh/oauth/token", params)
      ).data;
      this.accessToken = resp.access_token;
    } catch (error) {
      console.error(error);
      throw new Error("Could not retrieve access token");
    }
  }

  private async getRecentBeatmap(): Promise<RecentBeatmap> {
    try {
      return (
        await axios.get(
          `https://osu.ppy.sh/api/v2/users/${OSU_USER_ID}/scores/recent?limit=1`,
          {
            headers: {
              Authorization: `Bearer ${this.accessToken}`,
            },
          },
        )
      ).data[0];
    } catch (error) {
      console.error("Failed to fetch recent beatmap:", error);
      throw new Error("Could not retrieve recent beatmap data");
    }
  }

  private formatBeatmap(beatmap: RecentBeatmap): void {
    beatmap.beatmapset.title_unicode = beatmap.beatmapset.title_unicode.replace(
      /\s*(\(TV Size\)|\(TV edit\)|~TV Size~|\(Cut Ver\.\)|-TV Ver-|\(TV Size Ver\.\)|-TV MIX-|\(Movie Size\)|\(short ver\.\)|<TV\.Size Version>)/gi,
      "",
    );
    beatmap.created_at = Date.parse(beatmap.created_at as string) / 1000;
  }

  private listener(): void {
    const lasfmApi = new Scrobble();
    setInterval(async () => {
      const recentBeatmap: RecentBeatmap = await this.getRecentBeatmap();
      if (!recentBeatmap) return;
      if (recentBeatmap.id === this.lastScoreID) return;
      this.lastScoreID = recentBeatmap.id;
      this.formatBeatmap(recentBeatmap);
      const track: Track = {
        artist: recentBeatmap.beatmapset.artist_unicode,
        title: recentBeatmap.beatmapset.title_unicode,
        timestamp: recentBeatmap.created_at as number,
      };
      lasfmApi.scrobleTrack(track);
    }, 30000);
  }

  public async init(): Promise<void> {
    await this.getAcessToken();
    const recentBeatmap: RecentBeatmap = await this.getRecentBeatmap();
    if (recentBeatmap) {
      this.lastScoreID = recentBeatmap.id;
      this.formatBeatmap(recentBeatmap);
    }
    this.listener();
  }
}

const osu = new Osu();
osu.init();
