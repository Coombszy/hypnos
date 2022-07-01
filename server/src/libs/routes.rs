use actix_web::{error, get, post, web, Error, HttpResponse};
use chrono::Utc;
use futures_util::StreamExt as _;
use log::{debug, error};

use crate::libs::{
    structs::{State, SysState, WebError, WebHealth},
    utils::send_magic_packet,
};

use super::structs::TargetState;

const MAX_PAYLOAD_SIZE: usize = 262_144; // Max size of 256k

#[get("/health")]
async fn health(data: web::Data<State>) -> HttpResponse {
    debug!("Health request received");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(WebHealth {
            uptime: data.uptime(),
        })
}

#[post("/state")]
async fn set_state(
    data: web::Data<State>,
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
        TargetState::Off => println!("NOT IMPL"),
    }

    Ok(HttpResponse::NoContent().finish())
}
