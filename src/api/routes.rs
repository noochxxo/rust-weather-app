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

    // TODO: Get geolocation using the location data passed along with the request

    let (latitude, longitude, location) = geocode("&", &|result| {
        match result {
            Ok((latitude, longitude, location)) => {
                
                (latitude, longitude, location)
            }
            Err(error) => {
                println!("Error: {}", error);
                return (0.0, 0.0, error.to_owned())
            }
        }
    }).await.unwrap();

    // TODO: Get weather data
    forecast(latitude, longitude, &|result| {
        match result {
            Ok(forecast) => println!("Forecast: {}", forecast),
            Err(error) => println!("Error: {}", error),
        }
    }).await;

    HttpResponse::Ok().body(format!("{} is colder than a witches tit.", location))
}

pub async fn _404() -> impl Responder {
    HttpResponse::Ok().body("You made a wrong turn, muh guy.")
}