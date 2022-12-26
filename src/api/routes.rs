

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

    let name = json!({"name": "eric"});
    
    let body: String = match hb.render("views/index", &name ) {
        Ok(v) => v,
        Err(e) => "".to_owned(),
    };
    HttpResponse::Ok().body(body)
}

// #[get("/")]
// pub async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello, Bitches!")
// }

#[get("/weather")]
pub async fn weather() -> impl Responder {
    // TODO: Have location data passed along with the request

    let location_data = geocode("Saskatoon").await.unwrap();

    let feature = &location_data["features"][0];
    let latitude = feature["center"][1].as_f64().unwrap();
    let longitude = feature["center"][0].as_f64().unwrap();
    
    let weather_data = forecast(latitude, longitude).await.unwrap();

    // HttpResponse::Ok().content_type(value)

    // HttpResponse::Ok().body(format!("{:?}", weather_data))
    // let name = json!(weather_data);
    
    web::Json(weather_data.to_owned())
}

pub async fn _404() -> impl Responder {
    HttpResponse::Ok().body("You made a wrong turn, muh guy.")
}