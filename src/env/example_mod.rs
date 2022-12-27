pub mod keys {

pub struct APIKeys {
  pub weather: String,
  pub location: String,
}

pub fn get_env_keys() -> APIKeys {
  // set your api keys here
  let api_keys = APIKeys {
    weather: "secret_key".to_owned(),
    location: "secret_key".to_owned(),
  };
  api_keys
}
}