#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate r2d2;
#[macro_use]
extern crate log;
extern crate futures;
extern crate actix_web;

mod db;
mod models;
mod schema;
mod routes;
mod config_store;

use actix_web::{web, HttpServer, App};
use dotenv::dotenv;
use std::sync::Arc;
use crate::config_store::ConfigStore;
use actix::{Addr, Actor, System};

pub struct AppState {
    store: Arc<Addr<ConfigStore>>,
}

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    let sys = System::new("actix-http-server");

    let actor = Arc::new(
        ConfigStore::new(db::establish_connection()).start());

    let num_workers = std::env::var("NUM_WORKERS").unwrap_or("4".to_owned()).parse::<usize>().unwrap();
    let server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                store: actor.clone()
            })
            .route("/", web::get().to_async(routes::get))
            .route("/", web::post().to_async(routes::add))
            .route("/", web::put().to_async(routes::update))
            .route("/all", web::get().to_async(routes::all))
            .route("/", web::delete().to_async(routes::delete))
    })
        .keep_alive(10)
        .workers(num_workers);

    let port = std::env::var("PORT").unwrap_or("8080".to_owned());

    // start the server
    server.bind(format!("0.0.0.0:{}", port))
        .unwrap()
        .start();

    info!("Started http server: 0.0.0.0:{}", port);

    let _ = sys.run();
}
