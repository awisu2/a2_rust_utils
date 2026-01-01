use std::time::{SystemTime, UNIX_EPOCH};

pub struct Timestamp {}

impl Timestamp {
    pub fn from_system_time(system_time: SystemTime) -> u64 {
        system_time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
    }
}