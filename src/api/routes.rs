use crate::utils::{
    geocode::geocode,
    forecast::forecast,
    };

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Bitches!")
}

#[get("/weather")]
pub async fn weather() -> impl Responder {
    // TODO: Have location data passed along with the request

    let location_data = geocode("Saskatoon").await.unwrap();

    let feature = &location_data["features"][0];
    let latitude = feature["center"][1].as_f64().unwrap();
    let longitude = feature["center"][0].as_f64().unwrap();
    
    let weather_data = forecast(latitude, longitude).await.unwrap();

    HttpResponse::Ok().body(format!("{:?}", weather_data))
}

pub async fn _404() -> impl Responder {
    HttpResponse::Ok().body("You made a wrong turn, muh guy.")
}