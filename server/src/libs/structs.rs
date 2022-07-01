use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

// TOML Data on loaded on startup
#[derive(Deserialize, Clone)]
pub struct TOMLData {
    pub config: Config,
}

// Config data stored within TOML Data
#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub schedule_enabled: bool,
    pub schedule_source: String,
    pub web_enabled: bool,
    pub web_host: String,
    pub web_port: u16,
    pub write_logs: bool,
    pub write_logs_file: String,
}

// Implement functionality for Config struct
impl Config {
    // Returns struct as JSON
    pub fn display(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Returns struct as pretty JSON
    pub fn display_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

// Actix Application global state
pub struct State {
    pub start_time: DateTime<Utc>,
}
// Global state impls
impl State {
    // Returns current uptime using `start_time`
    pub fn uptime(&self) -> String {
        let duration: Duration = Utc::now() - self.start_time;

        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        return format!("{hours:02}:{minutes:02}:{seconds:02}",);
    }
}

// Web reoute 'health' response body
#[derive(Serialize)]
pub struct WebHealth {
    pub uptime: String,
}