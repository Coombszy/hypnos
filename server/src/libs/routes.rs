use actix_web::{
    error, get, post,
    web::{self, Query},
    App, Error, HttpResponse,
};
use chrono::Utc;
use futures_util::StreamExt as _;
use hypnos_library::{
    structs::{StateQuery, SysState, TargetState},
    utils::generic_mac_address,
};
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
        TargetState::On => send_magic_packet(&state.get_mac_address().unwrap())?,
        TargetState::Off => push_new_state(data, state),
    }

    Ok(HttpResponse::NoContent().finish())
}

fn push_new_state(data: web::Data<AppState>, payload_state: SysState) {
    let mut states = data.pending_states.lock().unwrap();
    // TODO: This needs validation
    states.push(payload_state);
}

#[get("/states")]
async fn get_states(data: web::Data<AppState>) -> HttpResponse {
    let states = data.pending_states.lock().unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(states.clone())
}

#[post("/query_state")]
async fn fetch_state(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    debug!("State fetch request received");

    // Convert payload stream into useful object
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("payload overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let state_query = match serde_json::from_slice::<StateQuery>(&body) {
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

    let mut state_cleanup = Vec::<usize>::new();
    let mut states = data.pending_states.lock().unwrap();
    let mut return_state: Option<SysState> = None;

    // Get state to return and a list of states to cleanup
    for mac_address in &state_query.mac_addresses {
        for (i, state) in states.iter().enumerate() {
            if state.get_mac_address() == generic_mac_address(mac_address) {
                state_cleanup.push(i);
                return_state = Some(state.clone());
            }
        }
    }

    // Reverse state_cleanup list to resolve indexing issues due to shifting indexes
    state_cleanup.reverse();
    // If state found, cleanup and respond
    if !state_cleanup.is_empty() && return_state.is_some() {
        for target in state_cleanup {
            states.remove(target);
        }
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(return_state.unwrap()))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}
