use std::sync::{Mutex, Arc};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Info{
    id:usize
}
#[derive(Serialize, Deserialize)]
struct CreatePlayList{
    name: String
}

#[derive(Debug, Serialize, Clone)]
pub struct Song{
    pub name: String,
    pub author: String,
    pub duration_ms: u16
}

#[derive(Debug, Serialize, Clone)]
pub struct Playlist{
    pub name: String,
    pub songs: Vec<Song>
}

pub struct State{
    pub playlist: Arc<Mutex<Vec<Playlist>>>
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("1.0.0")
}

#[get("/playlist")]
async fn playlist(data: web::Data<State>) -> impl Responder {
  let playlist = data.playlist.lock().expect("incorrect state");
    web::Json(playlist.clone())
}

#[get("/playlist/{id}")]
async fn get_playlist(info: web::Path<Info>, data: web::Data<State>) -> impl Responder {
    let playlists = data.playlist.lock().expect("incorrect state");
    let pl = playlists[info.id].clone();
    web::Json(pl)
}

#[post("/playlist")]
async fn create_playlist(new_play: web::Json<CreatePlayList>, data: web::Data<State>) -> impl Responder {
    let mut playlists = data.playlist.lock().expect("incorrect state");
    let pl = Playlist{
        name: new_play.name.clone(),
        songs: vec![]
    };
    playlists.push(pl.clone());
    web::Json(pl)
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(version);
    cfg.service(playlist);  
    cfg.service(get_playlist);  
    cfg.service(create_playlist);  
}
