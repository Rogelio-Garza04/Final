mod job;
mod worker;
mod utils;

use job::{Job, WebsiteStatus};
use utils::{load_urls_from_file, format_status_as_json};
use worker::run_job;

use std::env;
use std::fs::write;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut urls: Vec<String> = Vec::new();
    let mut file: Option<String> = None;
    let mut workers = num_cpus::get();
    let mut timeout_secs = 5;
    let mut retries = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i < args.len() {
                    file = Some(args[i].clone());
                }
            }
            "--workers" => {
                i += 1;
                if i < args.len() {
                    workers = args[i].parse().unwrap_or(workers);
                }
            }
            "--timeout" => {
                i += 1;
                if i < args.len() {
                    timeout_secs = args[i].parse().unwrap_or(timeout_secs);
                }
            }
            "--retries" => {
                i += 1;
                if i < args.len() {
                    retries = args[i].parse().unwrap_or(retries);
                }
            }
            arg => {
                if !arg.starts_with("--") {
                    urls.push(arg.to_string());
                }
            }
        }
        i += 1;
    }

    if let Some(path) = file {
        match load_urls_from_file(&path) {
            Ok(mut file_urls) => urls.append(&mut file_urls),
            Err(e) => {
                eprintln!("Failed to read file {}: {}", path, e);
                std::process::exit(1);
            }
        }
    }

    if urls.is_empty() {
        eprintln!("Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]");
        std::process::exit(2);
    }

    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (result_tx, result_rx) = mpsc::channel::<WebsiteStatus>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    for _ in 0..workers {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();
        thread::spawn(move || {
            loop {
                let job = {
                    let lock = job_rx.lock().unwrap();
                    lock.recv()
                };

                match job {
                    Ok(job) => {
                        let result = run_job(job);
                        result_tx.send(result).unwrap();
                    }
                    Err(_) => break, 
                }
            }
        });
    }

    let timeout = Duration::from_secs(timeout_secs);
    for url in &urls {
        job_tx
            .send(Job {
                url: url.clone(),
                timeout,
                retries,
            })
            .unwrap();
    }

    drop(job_tx); 

    let mut statuses = Vec::new();
    for _ in 0..urls.len() {
        if let Ok(status) = result_rx.recv() {
            println!("{:?}", status);
            statuses.push(status);
        }
    }

    let json = format!(
        "[\n{}\n]",
        statuses
            .iter()
            .map(format_status_as_json)
            .collect::<Vec<_>>()
            .join(",\n")
    );

    if let Err(e) = write("status.json", json) {
        eprintln!("Failed to write status.json: {}", e);
    }
}
