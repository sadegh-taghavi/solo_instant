use redis::Commands;
use actix::prelude::*;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::RwLock;
use crate::handler;

pub struct AppState {
    pub conf : super::config::Config,
    pub redis: RwLock<redis::Connection>,
    pub connections: Arc<Mutex<HashMap<String, Addr<handler::WebSocketConnection>>>>,
}
#[actix_web::main]
pub async fn init(conf : super::config::Config) -> std::io::Result<()> {
   
    let redis_result = redis::Client::open(conf.clone().redis.address);
    if redis_result.is_err() {
        panic!("error connecting to redis {}", redis_result.as_ref().unwrap_err())
    }
    let con_result = redis_result.unwrap().get_connection();
    if con_result.is_err() {
        panic!("error in redis connection")
    }
    let mut con = con_result.unwrap();
    let result: Result<(), redis::RedisError> = con.set_ex("test", "ok", TryInto::<usize>::try_into(10).unwrap());
    if result.is_err() {
        panic!("error in redis connection")
    }

    let app_state = web::Data::new(AppState { 
        conf: conf.clone(),
        redis: con.into(),
        connections: Arc::new(Mutex::new(HashMap::new())),
    });

    
    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        .route("/api/v1/health", web::get().to(handler::health))
        .route("/api/v1/info", web::get().to(handler::info))
        .route("/api/v1/ws", web::get().to(handler::websocket_index))
    })
    .bind(conf.server.address)?
    .run()
    .await
}