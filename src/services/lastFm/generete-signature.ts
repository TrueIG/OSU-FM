import { LASTFM_SECRET } from "@main/verify-env";
import type { LastFmParamsAuth, LastFmParamsScrobble } from "@types";
import crypto from "crypto";

export default function genereteSignature<
  T extends LastFmParamsAuth | LastFmParamsScrobble,
>(params: T) {
  const sortedKeys: string[] = Object.keys(params).sort();
  let stringToHash: string = "";

  sortedKeys.forEach((key: string) => {
    stringToHash += key + params[key as keyof T];
  });

  stringToHash += LASTFM_SECRET;

  const apiSig: string = crypto
    .createHash("md5")
    .update(stringToHash)
    .digest("hex");

  params.api_sig = apiSig;
}
