use crate::job::WebsiteStatus;
use std::fs;
use std::io::{self, BufRead};

pub fn load_urls_from_file(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .collect())
}

pub fn format_status_as_json(status: &WebsiteStatus) -> String {
    let status_field = match &status.action_status {
        Ok(code) => format!("\"status\": {}", code),
        Err(e) => format!("\"error\": \"{}\"", e),
    };

    format!(
        "{{ \"url\": \"{}\", {}, \"response_time_ms\": {}, \"timestamp\": {} }}",
        status.url,
        status_field,
        status.response_time.as_millis(),
        status.timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    )
}