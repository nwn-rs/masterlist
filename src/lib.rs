use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
  pub required: bool,
  pub hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nwsync {
  pub manifests: Vec<Manifest>,
  pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
  #[serde(rename = "first_seen")]
  pub first_seen: i64,
  #[serde(rename = "last_advertisement")]
  pub last_advertisement: i64,
  #[serde(rename = "session_name")]
  pub session_name: String,
  #[serde(rename = "module_name")]
  pub module_name: String,
  #[serde(rename = "module_description")]
  pub module_description: String,
  pub passworded: bool,
  #[serde(rename = "min_level")]
  pub min_level: i64,
  #[serde(rename = "max_level")]
  pub max_level: i64,
  #[serde(rename = "current_players")]
  pub current_players: i64,
  #[serde(rename = "max_players")]
  pub max_players: i64,
  pub build: String,
  pub rev: i64,
  pub pvp: i64,
  pub servervault: bool,
  pub elc: bool,
  pub ilr: bool,
  #[serde(rename = "one_party")]
  pub one_party: bool,
  #[serde(rename = "player_pause")]
  pub player_pause: bool,
  pub os: i64,
  pub language: i64,
  #[serde(rename = "game_type")]
  pub game_type: i64,
  pub latency: i64,
  pub host: String,
  pub port: i64,
  #[serde(rename = "kx_pk")]
  pub kx_pk: Option<String>,
  #[serde(rename = "sign_pk")]
  pub sign_pk: Option<String>,
  pub nwsync: Option<Nwsync>,
  pub connecthint: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Me {
  pub address: String,
  pub servers: Vec<Server>,
}

const BASE_URL: &str = "https://api.serverlist.tf/v1/";

pub async fn get_my_servers() -> Result<Me, reqwest::Error> {
  let resp = reqwest::get(format!("{}/me", BASE_URL)).await?.json::<Me>().await?;
  Ok(resp)
}

pub async fn get_servers() -> Result<Vec<Server>, reqwest::Error> {
  let response = match reqwest::get(format!("{}/servers", BASE_URL)).await {
    Ok(response) => response,
    Err(e) => return Err(e),
  };

  let json = match response.json::<Vec<Server>>().await {
    Ok(json) => json,
    Err(e) => return Err(e),
  };

  Ok(json)
}

pub async fn get_servers_by_public_key(public_key: String) -> Result<Vec<Server>, reqwest::Error> {
  let response = match reqwest::get(format!("{}/servers/{}", BASE_URL, public_key)).await {
    Ok(response) => response,
    Err(e) => return Err(e),
  };

  let json = match response.json::<Vec<Server>>().await {
    Ok(json) => json,
    Err(e) => return Err(e),
  };

  Ok(json)
}

pub async fn get_servers_by_ip_and_port(ip: String, port: i32) -> Result<Vec<Server>, reqwest::Error> {
  let url = format!("{}/servers/{}/{}", BASE_URL, ip, port);

  let response = match reqwest::get(url).await {
    Ok(response) => response,
    Err(e) => return Err(e),
  };

  let json = match response.json::<Vec<Server>>().await {
    Ok(json) => json,
    Err(e) => return Err(e),
  };

  Ok(json)
}
