use actix_web::{error, get, post, web, Error, HttpResponse, App};
use chrono::Utc;
use futures_util::StreamExt as _;
use hypnos_library::structs::{TargetState, SysState};
use log::{debug, error};

use crate::libs::{
    structs::{AppState, WebError, WebHealth},
    utils::send_magic_packet,
};

const MAX_PAYLOAD_SIZE: usize = 262_144; // Max size of 256k

#[get("/health")]
async fn health(data: web::Data<AppState>) -> HttpResponse {
    debug!("Health request received");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(WebHealth {
            uptime: data.uptime(),
        })
}

#[post("/state")]
async fn set_state(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    debug!("Status request received");

    // Convert payload stream into useful object
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("payload overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let state = match serde_json::from_slice::<SysState>(&body) {
        Ok(n) => n,
        Err(e) => {
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .json(WebError {
                    timestamp: Utc::now().to_rfc3339(),
                    error: format!("failed to parse json. {}", e),
                }));
        }
    };

    match state.target_state {
        TargetState::On => send_magic_packet(&state.get_mac_address())?,
        TargetState::Off => push_new_state(data, state),
    }

    Ok(HttpResponse::NoContent().finish())
}

fn push_new_state(data: web::Data<AppState>, payload_state: SysState) {
    let mut states = data.pending_states.lock().unwrap();
    states.push(payload_state);

}

#[get("/states")]
async fn get_states(
    data: web::Data<AppState>
) -> HttpResponse {

    let states = data.pending_states.lock().unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(states.clone())
}

