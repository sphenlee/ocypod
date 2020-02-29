//! HTTP handlers for the `/queue` endpoints.

use log::error;
use actix_web::{self, HttpResponse};
use actix_web::web::{Data, Path, Json};

use crate::actors::application;
use crate::models::{ApplicationState, job, queue, OcyError};

/// Handle `GET /queue` requests to get a JSON list of all existing queues.
///
/// # Returns
///
/// * 200 - JSON response containing list of queue names.
pub async fn index(data: Data<ApplicationState>) -> HttpResponse {
    let res = data.redis_addr.send(application::GetQueueNames).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(queue_names) => HttpResponse::Ok().json(queue_names),
        Err(OcyError::RedisConnection(err)) => {
            error!("Failed to fetch queue names: {}", err);
            HttpResponse::ServiceUnavailable().body(err.to_string())
        },
        Err(err)        => {
            error!("Failed to fetch queue names: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

/// Handles `PUT /queue/{queue_name}` requests.
pub async fn create_or_update(
    path: Path<String>,
    json: Json<queue::Settings>,
    data: Data<ApplicationState>,
) -> Result<HttpResponse, ()> {
    let queue_name = path.into_inner();
    let queue_settings = json.into_inner();
    let res = data.redis_addr.send(application::CreateOrUpdateQueue(queue_name.to_owned(), queue_settings)).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(true)  => Ok(HttpResponse::Created()
            .header("Location", format!("/queue/{}", queue_name))
            .finish()),
        Ok(false) => Ok(HttpResponse::NoContent()
            .reason("Queue setting updated")
            .header("Location", format!("/queue/{}", queue_name))
            .finish()),
        Err(OcyError::BadRequest(msg)) => Ok(HttpResponse::BadRequest().body(msg)),
        Err(OcyError::RedisConnection(err)) => {
            error!("[queue:{}] failed to create/update queue: {}", &queue_name, err);
            Ok(HttpResponse::ServiceUnavailable().body(err.to_string()))
        },
        Err(err)  => {
            error!("[queue:{}] failed to create/update queue: {}", &queue_name, err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        },
    }
 }

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
pub async fn delete(
    path: Path<String>,
    data: Data<ApplicationState>
) -> Result<HttpResponse, ()> {
    let queue_name = path.into_inner();
    let res = data.redis_addr.send(application::DeleteQueue(queue_name.clone())).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(true)  => Ok(HttpResponse::NoContent().reason("Queue deleted").finish()),
        Ok(false) => Ok(HttpResponse::NotFound().reason("Queue not found").finish()),
        Err(OcyError::BadRequest(msg)) => Ok(HttpResponse::BadRequest().body(msg)),
        Err(OcyError::RedisConnection(err)) => {
            error!("[queue:{}] failed to delete queue: {}", &queue_name, err);
            Ok(HttpResponse::ServiceUnavailable().body(err.to_string()))
        },
        Err(err)  => {
            error!("[queue:{}] failed to delete queue: {}", &queue_name, err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        },
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
pub async fn settings(
    path: Path<String>,
    data: Data<ApplicationState>
) -> Result<HttpResponse, ()> {
    let queue_name = path.into_inner();
    let res = data.redis_addr.send(application::GetQueueSettings(queue_name.clone())).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(summary) => Ok(HttpResponse::Ok().json(summary)),
        Err(OcyError::NoSuchQueue(_)) => Ok(HttpResponse::NotFound().into()),
        Err(OcyError::RedisConnection(err)) => {
            error!("[queue:{}] failed to fetch queue summary: {}", &queue_name, err);
            Ok(HttpResponse::ServiceUnavailable().body(err.to_string()))
        },
        Err(err)    => {
            error!("[queue:{}] failed to fetch queue summary: {}", &queue_name, err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        },
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
pub async fn size(
    path: Path<String>,
    data: Data<ApplicationState>
) -> Result<HttpResponse, ()> {
    let queue_name = path.into_inner();
    let res = data.redis_addr.send(application::GetQueueSize(queue_name.clone())).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(size) => Ok(HttpResponse::Ok().json(size)),
        Err(OcyError::NoSuchQueue(_)) => Ok(HttpResponse::NotFound().into()),
        Err(OcyError::RedisConnection(err)) => {
            error!("[queue:{}] failed to fetch queue size: {}", &queue_name, err);
            Ok(HttpResponse::ServiceUnavailable().body(err.to_string()))
        },
        Err(err)    => {
            error!("[queue:{}] failed to fetch queue size: {}", &queue_name, err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        },
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
pub async fn job_ids(
    path: Path<String>,
    data: Data<ApplicationState>
) -> Result<HttpResponse, ()> {
    let queue_name = path.into_inner();
    let res = data.redis_addr.send(application::GetQueueJobIds(queue_name.clone())).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(size) => Ok(HttpResponse::Ok().json(size)),
        Err(OcyError::NoSuchQueue(_)) => Ok(HttpResponse::NotFound().into()),
        Err(OcyError::RedisConnection(err)) => {
            error!("[queue:{}] failed to fetch queue size: {}", &queue_name, err);
            Ok(HttpResponse::ServiceUnavailable().body(err.to_string()))
        },
        Err(err)    => {
            error!("[queue:{}] failed to fetch queue size: {}", &queue_name, err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        },
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
pub async fn create_job(
    path: Path<String>,
    json: Json<job::CreateRequest>,
    data: Data<ApplicationState>,
) -> Result<HttpResponse, ()> {
    let queue_name = path.into_inner();
    let job_req = json.into_inner();
    let res = data.redis_addr.send(application::CreateJob(queue_name.clone(), job_req)).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(job_id) => Ok(HttpResponse::Created()
            .header("Location", format!("/job/{}", job_id))
            .json(job_id)),
        Err(OcyError::NoSuchQueue(_))  => Ok(HttpResponse::NotFound().reason("Queue Not Found").finish()),
        Err(OcyError::BadRequest(msg)) => Ok(HttpResponse::BadRequest().body(msg)),
        Err(OcyError::RedisConnection(err)) => {
            error!("[queue:{}] failed to create new job: {}", &queue_name, err);
            Ok(HttpResponse::ServiceUnavailable().body(err.to_string()))
        },
        Err(err)                       => {
            error!("[queue:{}] failed to create new job: {}", &queue_name, err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
pub async fn next_job(
    path: Path<String>,
    data: Data<ApplicationState>
) -> HttpResponse {
    let queue_name = path.into_inner();
    let res = data.redis_addr.send(application::NextJob(queue_name.clone())).await;

    let msg = match res {
        Ok(msg) => msg,
        Err(err) => Err(OcyError::Internal(err.to_string())),
    };
    match msg {
        Ok(Some(job)) => HttpResponse::Ok().json(job),
        Ok(None) => match &data.config.server.next_job_delay {
            Some(delay) if !delay.is_zero() => {
                actix_rt::time::delay_for(delay.0).await;
                HttpResponse::NoContent().into()
            },
            _ => HttpResponse::NoContent().into(),
        },
        Err(OcyError::NoSuchQueue(_)) => HttpResponse::NotFound().into(),
        Err(OcyError::RedisConnection(err)) => {
            error!("[queue:{}] failed to fetch next job: {}", &queue_name, err);
            HttpResponse::ServiceUnavailable().body(err.to_string())
        },
        Err(err) => {
            error!("[queue:{}] failed to fetch next job: {}", &queue_name, err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
