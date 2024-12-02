use crate::commandline_args::CommandlineArgs;
use clap::Parser;
use log::{error, info, warn};
use std::path::PathBuf;
use std::process::exit;

mod commandline_args;
mod curseforge_api;
mod pack_archive;
mod pack_manifest;
mod mod_type;
mod project_structure;
mod mod_file;

#[tokio::main]
async fn main() {
    let start_time = std::time::SystemTime::now();
    let args = CommandlineArgs::parse();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Starting unfuck-curseforge");
    warn!("This tool is not affiliated with CurseForge in any way, in fact we strongly dislike curseforge's bullshit!");

    if args.id.is_none() && args.file.is_none() {
        error!("You must specify a url or file to download");
        exit(1);
    }
    let mut file: Option<PathBuf> = None;

    if let Some(project_id) = args.id {
        file = Some(
            match curseforge_api::download_latest_pack_archive(project_id).await {
                Ok(file) => file,
                Err(err) => {
                    error!("Unable to download the latest pack archive: {}", err);
                    exit(1);
                }
            },
        );
    }

    if let Some(file_string) = args.file {
        file = Some(PathBuf::from(file_string));
    }
    if let Some(file) = file {
        let (tmp, manifest) = match pack_archive::process_archive(file, args.parallel_downloads, args.validate,args.validate_if_size_less_than).await {
            Ok(output) => output,
            Err(err) => {
                error!("Unable to process the pack archive: {}", err);
                exit(1);
            }
        };


        let mods = tmp.join("mods");
        let overrides = tmp.join("overrides");

        if let Some(output_path) = args.output {
            let output_path = output_path.join(format!(
                "{}-{}",
                manifest.name,
                std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            ));
            match pack_archive::copy_to_output(mods, overrides, output_path) {
                Ok(output) => {
                    info!("Pack copied to {}", output.display());
                }
                Err(err) => {
                    error!("Unable to copy pack to output: {}", err);
                }
            };
        }
    } else {
        error!("File not provided!");
        exit(1);
    }

    let end_time = std::time::SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    info!("Finished in {} seconds", duration.as_secs());
}
