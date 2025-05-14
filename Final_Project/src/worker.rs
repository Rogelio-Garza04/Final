use crate::job::{Job, WebsiteStatus};
use std::time::{Instant, Duration, SystemTime};
use reqwest::blocking::Client;
use std::thread;

pub fn run_job(job: Job) -> WebsiteStatus {
    let start = Instant::now();
    let mut last_err = String::new();

    for _ in 0..=job.retries {
        let client = Client::builder()
            .timeout(job.timeout)
            .build()
            .unwrap();

        match client.get(&job.url).send() {
            Ok(resp) => {
                return WebsiteStatus {
                    url: job.url,
                    action_status: Ok(resp.status().as_u16()),
                    response_time: start.elapsed(),
                    timestamp: SystemTime::now(),
                };
            }
            Err(e) => {
                last_err = e.to_string();
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    WebsiteStatus {
        url: job.url,
        action_status: Err(last_err),
        response_time: start.elapsed(),
        timestamp: SystemTime::now(),
    }
}
