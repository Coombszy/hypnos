use std::{env, str::FromStr};

use hypnos_library::draw_start_screen;

mod libs;
use libs::structs::State;

use actix_web::{web, App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;
use log::{info, LevelFilter};
use simplelog::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    startup();

    // Start Web
    let host: String = "0.0.0.0".to_string();
    let port: u16 = 4050;
    info!("Starting web server, listening on {host}:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                start_time: Utc::now(),
            }))
            .service(libs::routes::health)
    })
    .bind((host, port))?
    .run()
    .await

    // Testing
    /*
    let mac_address: [u8; 6] = [0x00, 0x23, 0x24, 0x82, 0x10, 0x46];

    let magic_packet = wake_on_lan::MagicPacket::new(&mac_address);

    magic_packet.send();

    println!("Sent!");
    */
}

fn startup() {
    draw_start_screen(&env!("CARGO_PKG_NAME").to_string());

    // Init environment vars from .env file if available
    dotenv().ok();

    // Init logging
    // Is NOTIFY_LOG_LEVEL in environment vars
    let level: LevelFilter = if env::var("HYPNOS_LOG_LEVEL").is_err() {
        LevelFilter::Info
    } else {
        LevelFilter::from_str(env::var("HYPNOS_LOG_LEVEL").unwrap().as_str()).unwrap()
    };

    // Create custom config
    let mut config: ConfigBuilder = simplelog::ConfigBuilder::default();
    config.set_time_format_custom(format_description!(
        "[hour]:[minute]:[second] [day]/[month]/[year]"
    ));
    CombinedLogger::init(vec![TermLogger::new(
        level,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
