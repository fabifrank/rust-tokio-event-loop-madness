extern crate actix_web;
extern crate serde_json;
extern crate actix_rt;
extern crate hyper;

use serde_json::{Value, json};
use hyper::{Client, Uri, Body, Request};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_rt::System;
use actix_web::client;
use futures::future::{Future, lazy};

fn main() {
    println!("Start server...");
    listen();
}

pub fn listen() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/push").route(web::post().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}


fn index(item: web::Json<Value>) -> HttpResponse {
    println!("model: {:?}", &item);
    send(json!({
        "hello": "world"
    }));

    HttpResponse::Ok().json(item.0) // <- send response
}



pub fn send(mut data: serde_json::Value) {
    println!("# Start running log post future...");

    System::new("test").block_on(lazy(|| {
        let req = Request::builder()
            .method("POST")
            .uri("http://localhost:8888")
            .body(Body::from(data.to_string()))
            .expect("request builder");

        let client = Client::new();
        let future = client.request(req)
        .and_then(|res| {
            println!("status: {}", res.status());
            Ok(())
        })
        .map_err(|err| {
            println!("error: {}", err);
        });
        return future;
    }));

    println!("# Finish running log post future")
}