extern crate reqwest;
use std::env;

pub async fn forecast(latitude:f64, longitude: f64) 
  -> Result<serde_json::Value, reqwest::Error> { 

  let weather_api_key = env::var("WEATHER_API_KEY"). expect("WEATHER_API_KEY must be set.");
  let url = format!("http://api.weatherstack.com/current?access_key={}&query={},{}&units=m", weather_api_key, latitude, longitude);

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