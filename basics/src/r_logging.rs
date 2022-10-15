use log::{debug, error, info, trace, warn};
use std::env;

use dotenvy::dotenv;
use env_logger;

pub fn main() {
    println!("\nlogging...");

    // 1. Use the `dotenvy` crate for env variables (make sure .env file has no ZSNBSP/BOM character)
    // 2. Use the `log` crate to log messages of different levels
    // 3. Use the `env_logger` crate

    dotenv().ok();
    env_logger::init();

    log_env_vars();
    output_all_levels();
}

fn log_env_vars() {
    for (key, value) in env::vars() {
        if key == "RUST_LOG" {
            println!("{} is set to {}", key, value);
        }
    }
}

fn output_all_levels() {
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}
