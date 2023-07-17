use serde::Serialize;
use actix_web::{Responder, web, HttpResponse};
use chrono::prelude::Utc;

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

// pub async fn get_profile(app_state: web::Data<crate::server::AppState>, claims: crate::model::TokenClaims) -> impl Responder {  

//     let result = crate::service::get_profile(&app_state, &claims)
//     .await;

//     if result.is_err() {

//         match result.as_ref().err().unwrap().status.as_str() {
//             "400" => return HttpResponse::BadRequest().json(result.as_ref().err().unwrap()),
//             "500" => return HttpResponse::InternalServerError().json(result.as_ref().err().unwrap()),
//             _ => return HttpResponse::InternalServerError().json(result.as_ref().err().unwrap())
//         }
//     }
//     HttpResponse::Ok().json(result.as_ref().ok().unwrap())

// }
