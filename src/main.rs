use log::{debug, error, info, warn};
use simple_logger::SimpleLogger;

mod bot;
mod config;
mod site;

fn main() {
    // Enable logging
    simple_logger::SimpleLogger::new()
        .env()
        .init()
        .expect("Unable to initialize logging.");
    debug!("Logging enabled successfully!");

    // Attempt to read config
}
