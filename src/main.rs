mod utils;
mod env;
mod api;

use api::{
    routes,
};
// us env::
use env::keys::set_env_keys;
use env::keys::API_Keys;

use std::env::set_var;
use actix_web::{App,web, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set api keys and environment variables
    let api_keys: API_Keys = set_env_keys();
    let map_key: String = "MAP_API_KEY".to_string();
    let weather_key: String = "WEATHER_API_KEY".to_string();
    set_var(map_key, api_keys.location);
    set_var(weather_key, api_keys.weather);

    HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::weather)
            .default_service(web::route().to(routes::_404))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
