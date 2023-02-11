use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use actix_rust_server::config::read_config;
use actix_rust_server::server;
use actix_rust_server::server::endpoints::{State, Playlist};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = read_config();
    let stack: Vec<Playlist> = vec![];
    let playlists = Arc::new(Mutex::new(stack));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State{playlist: playlists.clone()}))
            .service(web::scope("/api")
            .configure(server::endpoints::config))

    })
    .bind((config.host, config.port))?
    .run()
    .await
}
