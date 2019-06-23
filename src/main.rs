extern crate actix_web;
extern crate serde_json;
extern crate actix_rt;
extern crate hyper;
extern crate tokio;
use std::io;

use serde_json::{Value, json};
use hyper::{Client, Body, Request, Response, Error};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_rt::System;
use tokio::{spawn};
// use actix_web::client;
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
            .service(web::resource("/test").route(web::post().to(test)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}


fn index(item: web::Json<Value>) -> HttpResponse {
    println!("model: {:?}", &item);
    println!("before send");
    chain!(send(json!({
        "hello": "world"
    })), || {
        println!("after send");
    });
    HttpResponse::Ok().json(item.0)
}

fn test(item: web::Json<Value>) -> HttpResponse {
    println!("recevied test call!");
    println!("{:?}", &item);
    
    HttpResponse::Ok().json(item.0) // <- send response
}

// fn my_fut() -> Result<i32, i32> {
//     println!("Run future #2");
//     Ok(1)
// }

#[macro_export]
macro_rules! chain {
    ( $x:expr, $y:expr ) => ({
        let f = $x;
        let all = f.and_then(|res| {
                println!("status: {}", res.status());
                $y;
                Ok(())
            })
            .map_err(|err| {
                println!("error: {}", err);
            });
        spawn(lazy(move || all));
    });
}

pub fn send(data: serde_json::Value) -> impl Future<Item=Response<Body>, Error=Error>{
    println!("# Start running log post future...");

    let req = Request::builder()
        .method("POST")
        .uri("http://localhost:8080/test")
        .header("Content-Type", "application/json")
        .body(Body::from(data.to_string()))
        .expect("request builder");


    let client = Client::new();
    let future = client.request(req);
    return future;
    // let all = future.and_then(|res| {
    //         println!("status: {}", res.status());
    //         Ok(())
    //     })
    //     .map_err(|err| {
    //         println!("error: {}", err);
    //     });


    // spawn(lazy(move || all));

    // future.and_then(|res| {
    //     println!("after all");
    //     Ok(());
    // })

    // return future
    // return future;
    // println!("# Finish running log post future")
}