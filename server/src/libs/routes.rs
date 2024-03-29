use actix_web::{
    error, get, post,
    web::{self},
    Error, HttpResponse,
};
use chrono::Utc;
use futures_util::StreamExt as _;
use hypnos_library::{
    structs::{SysState, TargetState},
    utils::generic_mac_address,
};
use log::debug;

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
        TargetState::WolOn => send_magic_packet(&state.get_mac_address().unwrap())?,
        _ => push_new_state(data, state),
    }

    Ok(HttpResponse::NoContent().finish())
}

fn push_new_state(data: web::Data<AppState>, payload_state: SysState) {
    let mut states = data.pending_states.lock().unwrap();

    // Remove existing states with the same mac address
    states.retain(|state| state.mac_address != payload_state.mac_address);

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

#[get("/query_state/{mac_address}")]
async fn query_state(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    debug!("State query request received");

    let states = data.pending_states.lock().unwrap();
    let mut return_state: Option<SysState> = None;

    let mac_query = path.into_inner();

    // Get state to return and a list of states to cleanup
    for state in states.iter() {
        if state.get_mac_address() == generic_mac_address(&mac_query) {
            return_state = Some(state.clone());
        }
    }

    // Reverse state_cleanup list to resolve indexing issues due to shifting indexes
    // If state found, cleanup and respond
    if return_state.is_some() {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(return_state.unwrap()))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

#[get("/fetch_state/{mac_address}")]
async fn fetch_state(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    debug!("State fetch request received");

    let mut state_cleanup = Vec::<usize>::new();
    let mut states = data.pending_states.lock().unwrap();
    let mut return_state: Option<SysState> = None;

    let mac_query = path.into_inner();

    // Get state to return and a list of states to cleanup
    for (i, state) in states.iter().enumerate() {
        if state.get_mac_address() == generic_mac_address(&mac_query) {
            state_cleanup.push(i);
            return_state = Some(state.clone());
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
