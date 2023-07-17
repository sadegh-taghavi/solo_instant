use actix_web::{web, App, HttpServer};
use crate::handler;
use std::sync::RwLock;

pub struct AppState {
    pub conf : super::config::Config,
    pub redis: RwLock<redis::Connection>,
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

    let app_state = web::Data::new(AppState { 
        conf: conf.clone(),
        redis: con_result.unwrap().into()
    });

    
    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        .route("/api/v1/health", web::get().to(handler::health))
        .route("/api/v1/info", web::get().to(handler::info))
    })
    .bind(conf.server.address)?
    .run()
    .await
}