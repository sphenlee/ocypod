//! Handlers for getting general information about the Ocypod server as a whole.

use actix_web::{self, HttpRequest, HttpResponse};
use actix_web::web::Data;
use log::error;

use crate::actors::application;
use crate::models::{ApplicationState, OcyError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handles `GET /info` requests.
///
/// # Returns
///
/// * 200 - JSON containing summary of server information
pub async fn index(data: Data<ApplicationState>) -> Result<HttpResponse, ()> {
    let res = data.redis_addr.send(application::GetInfo).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(summary) => Ok(HttpResponse::Ok().json(summary)),
        Err(OcyError::RedisConnection(err)) => {
            error!("Failed to fetch summary data: {}", err);
            Ok(HttpResponse::ServiceUnavailable().body(err.to_string()))
        },
        Err(err)    => {
            error!("Failed to fetch summary data: {}", err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        },
    }
}

/// Handles `GET /info/version` requests.
pub fn version(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(VERSION)
}
