extern crate reqwest;

use std::env;


pub async fn forecast(latitude: f64, longitude:f64, callback: &dyn Fn(Result<(String), &str>)) {
  let weather_api_key = env::var("WEATHER_API_KEY"). expect("WEATHER_API_KEY must be set.");
  let url = format!("http://api.weatherstack.com/current?access_key={}&query={},{}&units=m", weather_api_key, latitude, longitude);

  let body = reqwest::get(url)
    .await
    .unwrap()
    .json::<serde_json::Value>()
    .await
    .unwrap();

  let mut weather_description: String = "".to_owned();
  let mut temperature: f64 = 0.0;

    if body["error"].is_null() {
      weather_description = body["current"]["weather_descriptions"][0].as_str().unwrap().to_owned();
      temperature = body["current"]["temperature"].as_f64().unwrap();
      // println!("{}", weather_description);
      // println!("{}", temperature);
    } else {
      callback(Err("Unable to find the location - weatherstack"))
    }

    callback(Ok(format!("{}. It is currently {} degrees out.", weather_description, temperature)));
}