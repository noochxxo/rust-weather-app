use std::collections::HashMap;

use crate::utils::{
    geocode::geocode,
    forecast::forecast,
    };

use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
    };
    use handlebars::Handlebars;
use serde_json::json;


#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {

    let page_data = json!({
        "name": "eric",
        "title": "Ugly ass Rust Weather App"
        });
    
    let body: String = match hb.render("views/index", &page_data ) {
        Ok(v) => v,
        Err(e) => "".to_owned(),
    };
    HttpResponse::Ok().body(body)
}

#[get("/weather")]
pub async fn weather(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // API request are made here, weather and location data are returned
    
    let location_data = geocode(query["address"].as_str()).await.unwrap();

    let feature = &location_data["features"][0];
    let latitude = feature["center"][1].as_f64().unwrap();
    let longitude = feature["center"][0].as_f64().unwrap();
    
    let weather_data = forecast(latitude, longitude).await.unwrap();
    
    web::Json(weather_data.to_owned())
}

pub async fn _404() -> impl Responder {
    HttpResponse::Ok().body("You made a wrong turn, muh guy.")
}