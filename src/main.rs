// main.rs

mod file_handler;
mod database;
mod logger;

use clap::{App, Arg};
use log::info;
use file_handler::process_file;
use database::Database;
use logger::init_logger;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize logger
    init_logger();
    
    // Configure CLI arguments
    let matches = App::new("File Receiver")
        .version("1.0")
        .author("Sandbox Developer")
        .arg(
            Arg::new("file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    info!("Received file path: {}", file_path);

    // Create database connection
    let db = Database::new("sandbox.db")?;
    
    // Process the received file
    process_file(file_path, &db)?;

    Ok(())
}
