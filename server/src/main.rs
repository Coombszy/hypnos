use hypnos_library::structs::{CargoPkgInfo, SysState};
use hypnos_library::utils::draw_start_screen;

mod libs;
use libs::structs::{AppState, TOMLData};

use actix_web::{web, App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, info, LevelFilter};
use simplelog::*;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::{env, str::FromStr};
use std::{vec};

use crate::libs::utils::load_config_toml;

const DATA_FOLDER: &str = "config/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let toml_data: TOMLData = startup();

    // Load Notifications
    // if toml_data.config.schedule_enabled {
    //     let notifications: Vec<Notification> =
    //         load_notifications(&toml_data.config.schedule_source);
    //     info!("Notifications loaded: {}", notifications.len());

    //     // Create scheduled notifications
    //     notification_scheduler(&notifications, toml_data.config.clone());
    // }

    // Create State change collection
    let pending_states = Arc::new(Mutex::new(Vec::<SysState>::new()));

    // Start Web
    let host: String = toml_data.clone().config.web_host;
    let port: u16 = toml_data.clone().config.web_port;
    info!("Starting web server, listening on {host}:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                start_time: Utc::now(),
                pending_states: pending_states.clone(),
            }))
            .service(libs::routes::health)
            .service(libs::routes::set_state)
            .service(libs::routes::get_states)
            .service(libs::routes::fetch_state)
    })
    .bind((host, port))?
    .run()
    .await
}

fn startup() -> TOMLData {
    draw_start_screen(&CargoPkgInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        authors: env!("CARGO_PKG_AUTHORS").to_string(),
    });

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

    toml_data
}
