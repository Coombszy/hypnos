
use actix_web::{error, get, post, web, Error, HttpResponse};
use log::{debug, error};

use crate::libs::{structs::{State, WebHealth}, utils::send_magic_packet};

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

#[get("/test")]
async fn test(data: web::Data<State>) -> HttpResponse {
    error!("TEST HIT! Booting temp test");

    send_magic_packet(&[0x00, 0x23, 0x24, 0x82, 0x10, 0x46]);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(WebHealth {
            uptime: data.uptime(),
        })
}