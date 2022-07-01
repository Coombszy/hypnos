use hypnos_library::draw_start_screen;

mod libs;
use libs::structs::{State, TOMLData};

use actix_web::{web, App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, error, info, LevelFilter};
use simplelog::*;

use std::fs::{self, File};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, str::FromStr};

const DATA_FOLDER: &str = "config/";

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

fn startup() -> TOMLData {
    draw_start_screen(&env!("CARGO_PKG_NAME").to_string());

    // Init environment vars from .env file
    dotenv().ok();

    // Load TOML Data for config
    let toml_data: TOMLData = load_config_toml(format!("{}hypnos.toml", &DATA_FOLDER));

    // Init logging
    // Is HYPNOS_LOG_LEVEL in environment vars
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
    if toml_data.config.write_logs {
        CombinedLogger::init(vec![
            TermLogger::new(
                level,
                config.build(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                level,
                config.build(),
                File::create(toml_data.config.write_logs_file.clone()).unwrap(),
            ),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![TermLogger::new(
            level,
            config.build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    }

    // Config validation
    debug!("Config loaded:\n{}", toml_data.config.display_pretty());
    // Do not start with Scheduled and Web disabled
    if !(toml_data.config.schedule_enabled) && !(toml_data.config.web_enabled) {
        error!("Scheduled and Web notifications cannot both be disabled!");
        exit(1);
    }

    toml_data
}

// Loads TOMLData struct from filename
pub fn load_config_toml(filename: String) -> TOMLData {
    // Load in raw string from config toml
    let toml_raw = match fs::read_to_string(&filename) {
        Ok(c) => c,
        // Failed to read file
        Err(e) => {
            error!("Could not read TOML file '{}'", &filename);
            error!("Error: {}", e);
            exit(1);
        }
    };
    // Convert to TOML struct
    let config_data: TOMLData = match toml::from_str(&toml_raw) {
        Ok(d) => d,
        // Failed to parse from String to TOMLData Struct
        Err(e) => {
            error!("Unable to load data from {}", &filename);
            error!("Error: {}", e);
            exit(1);
        }
    };
    config_data
}
