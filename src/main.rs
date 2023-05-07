use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Country {
    flag: String,
    region: String,
    subregion: String,
    languages: HashMap<String, String>,
    timezones: Vec<String>,
    population: i64,
    latlng: Vec<f32>,
    capital: Vec<String>,
    capital_info: HashMap<String, Vec<f32>>,
    tld: Vec<String>,
    flags: Flags,
    currencies: HashMap<String, Currencies>,
    name: Name,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Flags {
    png: String,
    svg: String,
    alt: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Currencies {
    name: String,
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Name {
    official: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res_json: Vec<Country> = reqwest::Client::new()
        .get("https://restcountries.com/v3.1/all?fields=name,capital,currencies,population,flag,languages,region,subregion,timezones,latlng,capitalInfo,tld,flags")
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}
