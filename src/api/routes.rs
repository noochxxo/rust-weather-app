
use std::f64;

use crate::utils::geocode::geocode;

use actix_web::{get, HttpResponse, Responder};
use serde_json::error;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Bitches!")
}

#[get("/weather")]
pub async fn weather() -> impl Responder {
    // TODO: Have location data passed along with the request

    // TODO: Get geolocation using the location data passed along with the request

    let (latitude, longitude, location, error) = geocode("Saskatoon", &|result| {
        match result {
            Ok((latitude, longitude, location)) => {
                
                (latitude, longitude, location, "".to_owned())
            }
            Err(error) => {
                println!("Error: {}", error);
                return (0.0, 0.0, "".to_string(), error.to_owned())
            }
        }
    }).await.unwrap();


    
    // TODO: Get weather data
    HttpResponse::Ok().body(format!("{} is colder than a witches tit.", location))
}

pub async fn _404() -> impl Responder {
    HttpResponse::Ok().body("You made a wrong turn, muh guy.")
}