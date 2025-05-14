use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub action_status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: SystemTime,
}

pub struct Job {
    pub url: String,
    pub timeout: Duration,
    pub retries: usize,
}