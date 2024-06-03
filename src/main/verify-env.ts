import log from "loglevel";

const Defaults = {
  OSU_CLIENT_ID: process.env.OSU_CLIENT_ID,
  OSU_CLIENT_SECRET: process.env.OSU_CLIENT_SECRET,
  LASTFM_API_KEY: process.env.LASTFM_API_KEY,
  LASTFM_SECRET: process.env.LASTFM_SECRET,
  OSU_USER_ID: "",
};

type EnvVariableName = keyof typeof Defaults;

const getEnvVariable = (name: EnvVariableName): string => {
  const value = process.env[name];
  if (!value && !Defaults[name]) {
    throw new Error(`Environment variable ${name} is not set`);
  }
  return value || Defaults[name];
};

const OSU_CLIENT_ID = getEnvVariable("OSU_CLIENT_ID");
const OSU_CLIENT_SECRET = getEnvVariable("OSU_CLIENT_SECRET");
const LASTFM_API_KEY = getEnvVariable("LASTFM_API_KEY");
const LASTFM_SECRET = getEnvVariable("LASTFM_SECRET");
const OSU_USER_ID = getEnvVariable("OSU_USER_ID");

const LASTFM_API_URL = "https://ws.audioscrobbler.com/2.0/";

console.log("test");

log.setLevel("INFO");

export {
  LASTFM_API_URL,
  OSU_USER_ID,
  LASTFM_SECRET,
  LASTFM_API_KEY,
  OSU_CLIENT_SECRET,
  OSU_CLIENT_ID,
  log,
};
