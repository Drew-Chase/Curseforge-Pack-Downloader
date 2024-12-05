#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use crate::commandline_args::CommandlineArgs;
use clap::Parser;
use curseforge_pack_downloader::CurseforgePackDownloader;
use log::{error, info, warn};
use std::env::set_var;
use std::process::exit;

mod commandline_args;
mod env;

#[tokio::main]
async fn main() {
    // Record the start time of the process
    let start_time = std::time::SystemTime::now();

    // Parse command line arguments
    let args = CommandlineArgs::parse();

    // Set up the logging environment
    set_var("RUST_LOG", "info");
    env_logger::init();

    // Log starting information about the tool
    info!("Starting unfuck-curseforge");
    warn!("This tool is not affiliated with CurseForge in any way, in fact we strongly dislike curseforge's bullshit!");

    // Attempt to read environment variables from the embedded env.ini file
    let env = match env::Env::new() {
        Ok(env) => env,
        Err(err) => {
            // Log an error and exit if env variables cannot be read
            error!("Unable to read environment variables from the env.ini file, make sure its filled out before building: {}", err);
            exit(1);
        }
    };

    // set the `CURSEFORGE_API_KEY` environment variable
    // this will be required by the `CurseforgePackDownloader`
    set_var("CURSEFORGE_API_KEY", &env.curseforge_api_key);

    // Create an instance of the `CurseforgePackDownloader` struct.
    let mut downloader = CurseforgePackDownloader::new();

    // Set downloader options based on input arguments
    downloader.set_validate(args.validate);
    downloader.set_parallel_downloads(args.parallel_downloads);

    // Set validation size limit if provided and validation is enabled
    if let Some(validate_if_less_than_bytes) = args.validate_if_size_less_than {
        downloader.set_validate_if_size_less_than(validate_if_less_than_bytes);
    }

    // Determine processing path based on input ID or file
    match if let Some(id) = args.id {
        downloader.process_id(id).await
    } else if let Some(file) = args.file {
        downloader.process_file(file).await
    } else {
        // Log an error if neither an ID nor a file is specified and exit
        error!("You must specify a url or file to download");
        exit(1);
    } {
        Ok(manifest) => manifest,
        Err(err) => {
            // Log an error and exit if processing fails
            error!("Failed to process pack: {}", err);
            exit(1);
        }
    };

    // Calculate and log the duration of the process
    let end_time = std::time::SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap_or_else(|err| {
        error!("Unable to get current time: {}", err);
        std::time::Duration::new(0, 0)
    });
    info!("Finished in {} seconds", duration.as_secs());
}
