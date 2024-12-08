use curseforge_pack_downloader::modpack_version_file::ModpackVersionFile;
use curseforge_pack_downloader::ProcessProgressResponse;
use log::{error, info};
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::convert::From;
use std::path::PathBuf;
use tauri::ipc::Channel;

/// Macro to parse the CURSEFORGE_API_KEY from environment variables.
///
/// This macro attempts to retrieve the API key from the environment, parse it,
/// and handle any potential errors during parsing. If successful, it returns
/// the parsed API key.
macro_rules! header_parsed_api_key {
    () => {
        match std::env::var("CURSEFORGE_API_KEY")
            .unwrap_or("".to_string())
            .parse()
        {
            Ok(key) => key, // Return the parsed API key if successful
            Err(err) => {
                // Log an error message if parsing fails
                error!("Failed to parse API key: {}", err);
                return Err("Failed to parse API key".into());
            }
        }
    };
}

#[tauri::command]
pub async fn search_modpacks(query: String) -> Result<String, String> {
    info!("Searching for modpacks with query {:?}", query);
    // Create a new HTTP client instance
    let client = Client::new();

    // Prepare the headers with the API key
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", header_parsed_api_key!());

    // Create the API request to get the files for the specified project ID
    let request = client
        .get( format!("https://api.curseforge.com/v1/mods/search?gameId=432&searchFilter={}&classId=4471&sortOrder=desc&pageSize=50",query))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let body = request.text().await.map_err(|e| e.to_string())?;
    Ok(body)
}

#[tauri::command]
pub async fn unpack(
    id: u64,
    pack_version: u64,
    output: String,
    on_event: Channel<ProcessProgressResponse>,
) -> Result<(), String> {
    let mut downloader = curseforge_pack_downloader::CurseforgePackDownloader::default();
    downloader.set_parallel_downloads(16);
    downloader.set_output_directory(&output);
    downloader.set_temp_directory(PathBuf::from(output).join("temp"));
    downloader.set_validate(true);
    downloader.set_validate_if_size_less_than(10000);
    downloader.set_pack_version(pack_version);

    match downloader
        .process_id(id, move |e| {
            on_event.send(e).unwrap();
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn unpack_file(
    file: String,
    output: String,
    on_event: Channel<ProcessProgressResponse>,
) -> Result<(), String> {
    let mut downloader = curseforge_pack_downloader::CurseforgePackDownloader::default();
    downloader.set_parallel_downloads(16);
    downloader.set_output_directory(&output);
    downloader.set_temp_directory(PathBuf::from(output).join("temp"));
    downloader.set_validate(true);

    match downloader
        .process_file(file, move |e| {
            on_event.send(e).unwrap();
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_pack_versions(id: u64) -> Result<Vec<ModpackVersionFile>, String> {
    curseforge_pack_downloader::curseforge_api::get_pack_versions(id)
        .await
        .map_err(|e| e.to_string())
}
