pub mod keys {

pub struct API_Keys {
  pub weather: String,
  pub location: String,
}

pub fn set_env_keys() -> API_Keys {
  let api_keys = API_Keys {
    weather: "".to_owned(),
    location: "".to_owned(),
  };
  api_keys
}
}

