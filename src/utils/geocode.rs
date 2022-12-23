extern crate reqwest;

use std::env;

// TODO: Show current time for searched location, and current location

// Is there a better way to get the location without using closures

pub async fn geocode(address: &str, callback: &dyn Fn(Result<(f64, f64, String), &str>) -> (f64, f64, String, String)) -> Result<(f64, f64, String, String), String>  {
  let map_api_key = env::var("MAP_API_KEY").expect("MAP_API_KEY must be set");
  let url = format!("https://api.mapbox.com/geocoding/v5/mapbox.places/{}.json?access_token={}&limit=1", address, map_api_key);


  let body = reqwest::get(url)
    .await
    .unwrap()
    .json::<serde_json::Value>()
    .await
    .unwrap();
   

  // TODO: refactor the following code
  // TODO: program panics when no location is found

  let mut latitude: f64 = 0.0;
  let mut longitude: f64 = 0.0;
  let mut location: String = "".to_owned();

  if body["features"].is_null() {
    callback(Err("Unable to connect to location services!"));
  } else if body["features"].as_array().unwrap().is_empty() {
    callback(Err("Unable to find location. Try another search."));
  } else {
     let feature = &body["features"][0];
     latitude = feature["center"][1].as_f64().unwrap();
     longitude = feature["center"][0].as_f64().unwrap();
     location = feature["place_name"].as_str().unwrap().to_owned();
     callback(Ok((latitude, longitude, location.to_owned())));
  }
  Ok((latitude, longitude, location, "".to_owned()))

}