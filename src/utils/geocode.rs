extern crate reqwest;

use std::env;

pub async fn geocode(address: &str) -> Result<serde_json::Value, reqwest::Error>  {
  let map_api_key = env::var("MAP_API_KEY").expect("MAP_API_KEY must be set");
  let url = format!("https://api.mapbox.com/geocoding/v5/mapbox.places/{}.json?access_token={}&limit=1", address, map_api_key);


  let body = reqwest::get(url)
    .await
    .unwrap()
    .json::<serde_json::Value>()
    .await;
  
  match body {
    Ok(v) => Ok(v),
    Err(e) => Err(e),
  }

}