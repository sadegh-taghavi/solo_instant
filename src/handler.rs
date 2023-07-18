use serde::Serialize;
use actix_web::{Responder, web, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use actix::prelude::*;
use chrono::prelude::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn health() -> impl Responder {
    #[derive(Debug, Serialize)]
    struct Health {
        status: String,
    }

    HttpResponse::Ok().json(Health {
        status: "ok".to_string(),
    })
}

pub async fn info() -> impl Responder {  
    #[derive(Debug, Serialize)]
    struct Info {
        info: String,
        version: String,
        time: String,
    }

    HttpResponse::Ok().json(Info {
        info: "Instant communication service".to_string(),
        version: "0.0.1".to_string(),
        time: Utc::now().to_string(),
    })
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection established");
        self.connections.lock().unwrap().insert(self.claims.sub.clone(), ctx.address());
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.connections.lock().unwrap().remove(&self.claims.sub.clone());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                info!("Received text message: {}", text);
                ctx.text(format!("You sent: {}", text));
            }
            Ok(ws::Message::Binary(bin)) => {
                info!("Received binary message: {:?}", bin);
                ctx.binary(bin);
            }
            _ => (),
        }
    }
}

pub struct WebSocketConnection {
    connections: Arc<Mutex<HashMap<String, Addr<WebSocketConnection>>>>,
    claims: crate::model::TokenClaims
}

impl WebSocketConnection {
    fn new(connections: Arc<Mutex<HashMap<String, Addr<WebSocketConnection>>>>, claims: crate::model::TokenClaims) -> Self {
        WebSocketConnection { connections, claims }
    }
}

pub async fn websocket_index(r: HttpRequest,stream: web::Payload,app_state: web::Data<crate::server::AppState>, claims: crate::model::TokenClaims) -> Result<HttpResponse, actix_web::Error> {
    let res = ws::start(WebSocketConnection::new(app_state.connections.clone(), claims), &r, stream);
    res
}
