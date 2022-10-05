use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use hypnos_library::structs::SysState;
use serde::{Deserialize, Serialize};

// TOML Data on loaded on startup
#[derive(Deserialize, Clone)]
pub struct TOMLData {
    pub config: Config,
}

// Config data stored within TOML Data
#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub web_host: String,
    pub web_port: u16,
    pub write_logs: bool,
    pub write_logs_file: String,
}

// Implement functionality for Config struct
impl Config {
    // Returns struct as pretty JSON
    pub fn display_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

// Actix Application global state
pub struct AppState {
    pub start_time: DateTime<Utc>,
    pub pending_states: Arc<Mutex<Vec<SysState>>>,
}
// Global state impls
impl AppState {
    // Returns current uptime using `start_time`
    pub fn uptime(&self) -> String {
        let duration: Duration = Utc::now() - self.start_time;

        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        return format!("{hours:02}:{minutes:02}:{seconds:02}",);
    }
}

// Web route 'health' response body
#[derive(Serialize)]
pub struct WebHealth {
    pub uptime: String,
}

// Reponse error
#[derive(Serialize)]
pub struct WebError {
    pub timestamp: String,
    pub error: String,
}
