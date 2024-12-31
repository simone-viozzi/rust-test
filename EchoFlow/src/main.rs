use log::info;

mod api;
mod tui;

use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging to a file
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("app.log").unwrap(),
    )
    .unwrap();

    info!("Starting application");

    // Start the API in a separate thread
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(api::start_api());
    });

    // Run the TUI in the main thread
    tui::start_tui().await?;

    info!("Application exiting");
    Ok(())
}
