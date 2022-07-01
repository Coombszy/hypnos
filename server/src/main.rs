use hypnos_library::draw_start_screen;

mod libs;
use libs::structs::{State, TOMLData};

use actix_web::{web, App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, error, info, LevelFilter};
use simplelog::*;

use std::fs::File;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{env, str::FromStr};

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

    // Start Web
    if toml_data.config.web_enabled {
        let host: String = toml_data.clone().config.web_host;
        let port: u16 = toml_data.clone().config.web_port;
        info!("Starting web server, listening on {host}:{port}");
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(State {
                    start_time: Utc::now(),
                }))
                .service(libs::routes::health)
                .service(libs::routes::test)
        })
        .bind((host, port))?
        .run()
        .await
    }
    // Keep app alive
    else {
        loop {
            thread::sleep(Duration::from_secs(10));
        }
    }
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
