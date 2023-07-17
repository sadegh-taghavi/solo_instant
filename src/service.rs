// use chrono::Duration;
// use chrono::prelude::Utc;

// use oauth2::{basic::BasicClient, TokenResponse};
// use jsonwebtoken::{encode, Header, EncodingKey};
// use reqwest::{Client, Url};
// use uuid::Uuid;

// use redis::Commands;

// // Alternatively, this can be oauth2::curl::http_client or a custom.
// use oauth2::{
//     AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
//     RevocationUrl, Scope, TokenUrl,
//     reqwest::async_http_client,
// };

// use crate::model::{ ErrorResponse};

// pub async fn get_profile(app_state: &crate::server::AppState, claims: &crate::model::TokenClaims) ->  Result<crate::model::ProfileInfo, crate::model::ErrorResponse> {

//     let result = sqlx::query_as::<_, crate::model::ProfileInfo>("SELECT uuid, email, name FROM users WHERE uuid = ?").bind(claims.sub.clone())
//             .fetch_all(&app_state.dbp)
//             .await;
//             if result.is_err() {
//                 error!("error in query {}", result.as_ref().err().unwrap() );
//                 return Result::Err( ErrorResponse { status: "500".to_string(), message: "".to_string() })   
//             }
    
//     let mut profile = crate::model::ProfileInfo{
//         uuid:String::from(""),
//         email: String::from(""),
//         name:String::from(""),
//     };
//     if result.as_ref().unwrap().len() > 0 {
//         profile = result.as_ref().unwrap().first().unwrap().clone();
//     }
//     Result::Ok( profile )
// }
