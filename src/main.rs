mod service;

// use self::models::*;
// use diesel::prelude::*;
// use diesel_demo::*;

use crate::service::shortner::generate_short_code;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header};
use serde::{Deserialize, Serialize};
use url_shortner::{create_url, establish_connection, get_url_by_id, list_url};

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct URL {
    key: String,
    url: String,
}

#[post("/shorten")]
async fn shorten(req_body: String) -> impl Responder {
    println!("{:?}", req_body);
    let deserialized: Body = serde_json::from_str(&req_body).unwrap();
    
    println!("body: {:?}", deserialized);
    let connection = &mut establish_connection();
    let rand_short_code: String = generate_short_code();

    let result= create_url(connection, &deserialized.url, &rand_short_code);
    let serialized = serde_json::to_string(&result).unwrap();
    println!("Result: {:?}", result);
    HttpResponse::Ok().body(serialized)
}

#[get("/url")]
async fn short_code() -> impl Responder {
    
    let connection = &mut establish_connection();


    let result=list_url(connection);
    let serialized = serde_json::to_string(&result).unwrap();

    HttpResponse::Ok().body(serialized)
}

#[get("/geturl/{short_code}")]
async fn redirector(path: web::Path<String>) -> impl Responder {
    println!("{:?}", path);
    let connection = &mut establish_connection();


    let result=get_url_by_id(connection, &path);

    HttpResponse::PermanentRedirect()
    .insert_header((header::LOCATION, result.url))
    .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(shorten).service(short_code).service(redirector))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
