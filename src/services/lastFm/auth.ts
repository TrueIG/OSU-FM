import {
  Format,
  type LastFmParamsAuth,
  type LastFmToken,
  MethodAuth,
} from "@types";
import { LASTFM_API_KEY, LASTFM_API_URL, log } from "@main/verify-env";
import axios from "axios";
import open from "open";
import { XMLParser } from "fast-xml-parser";
import genereteSignature from "./generete-signature";

const parser = new XMLParser();

class LastFmAuth {
  private token: string | undefined;

  private async getAuthToken() {
    const params: LastFmParamsAuth = {
      method: MethodAuth.GETTOKEN,
      api_key: LASTFM_API_KEY,
      format: Format.JSON,
    };

    genereteSignature(params);
    try {
      const resp: LastFmToken = (await axios.get(LASTFM_API_URL, { params }))
        .data;
      this.token = resp.token;
      open(
        `http://www.last.fm/api/auth/?api_key=${LASTFM_API_KEY}&token=${this.token}`,
      );
      log.info("You have 30 seconds to authorize.\n");
      log.info(
        `Please enter the link to authorize, if it did not open automatically:\nhttp://www.last.fm/api/auth/?api_key=${LASTFM_API_KEY}&token=${this.token} \n`,
      );
      log.info("Please wait until it is finished\n");
    } catch (error) {
      console.error(error);
    }
  }

  private async getSession() {
    const params: LastFmParamsAuth = {
      method: MethodAuth.GETSESSION,
      api_key: LASTFM_API_KEY,
      token: this.token,
    };

    genereteSignature(params);
    try {
      const resp = await axios.get(LASTFM_API_URL, { params });
      const respJSON: JSON = parser.parse(resp.data);
      await Bun.write(`${process.cwd()}/token.json`, JSON.stringify(respJSON));
    } catch (error) {
      console.error(error);
    }
  }

  public async init() {
    log.info("The authentication request is starting, please do not close.\n");
    this.getAuthToken();
    await Bun.sleep(30000);
    this.getSession();
    log.info("Done. Authorization was a success.");
  }
}

const auth = new LastFmAuth();
auth.init();
