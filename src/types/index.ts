export interface OsuAuthToken {
  access_token: string;
  expires_in: number;
  refresh_token: string;
}

export interface RecentBeatmap {
  id: number;
  created_at: string | number;
  beatmapset: {
    artist: string;
    artist_unicode: string;
    title: string;
    title_unicode: string;
  };
}

export enum GrantType {
  REFRESH = "refresh_token",
  CREDENTIAL = "client_credentials",
}

export interface OsuParamsAuth {
  grant_type: GrantType;
  client_id: string;
  client_secret: string;
  scope: "public";
}

export enum Format {
  JSON = "json",
  XML = "",
}

export enum MethodAuth {
  GETTOKEN = "auth.gettoken",
  GETSESSION = "auth.getSession",
}

export interface LastFmParamsAuth {
  method: MethodAuth;
  api_key: string;
  api_sig?: string;
  format?: Format;
  token?: string;
}

export interface LastFmParamsScrobble {
  method: "track.scrobble";
  api_key: string;
  api_sig?: string;
  artist: string;
  track: string;
  timestamp: number;
  sk: string;
}

export interface LastFmToken {
  token: string;
}

export interface Track {
  artist: string;
  title: string;
  timestamp: number;
}
