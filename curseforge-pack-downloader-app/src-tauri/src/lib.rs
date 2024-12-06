use log::error;
use std::env::set_var;
use std::process::exit;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod curseforge_api;
mod env;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up the logging environment
    set_var("RUST_LOG", "info");
    env_logger::init();

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

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            curseforge_api::search_modpacks,
            curseforge_api::unpack,
            curseforge_api::unpack_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
