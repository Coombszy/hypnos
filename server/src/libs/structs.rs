use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

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
