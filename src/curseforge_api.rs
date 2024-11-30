use crate::pack_manifest::Manifest;
use futures::future;
use log::{error, info};
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const API_KEY: &str = r#"$2a$10$qD2UJdpHaeDaQyGGaGS0QeoDnKq2EC7sX6YSjOxYHtDZSQRg04BCG"#;

pub async fn download_latest_pack_archive(project_id: u64) -> Result<PathBuf, Box<dyn Error>> {
    info!("Downloading the latest pack version");
    let client = reqwest::Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", API_KEY.parse().unwrap());

    let request = client
        .get(format!(
            "https://api.curseforge.com/v1/mods/{}/files",
            project_id
        ))
        .headers(headers);

    let response: Value = match request.send().await {
        Ok(response) => response.json().await?,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    let data: Vec<Value> = response.get("data").unwrap().as_array().unwrap().to_vec();
    let latest_file: Value = data.first().unwrap().clone();
    let download_url: String = latest_file
        .get("downloadUrl")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let file_name: String = latest_file
        .get("fileName")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let file_path = std::env::temp_dir().join(file_name);
    let mut file = File::create(&file_path)?;
    let response = reqwest::get(download_url).await?;

    let bytes = response.bytes().await?;
    file.write_all(&bytes)?;
    Ok(file_path)
}

pub async fn download_mods_from_manifest(
    manifest: &Manifest,
    directory: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    info!("Downloading mods from manifest");
    create_dir_all(&directory)?;

    // Collect all download tasks into a vector
    let download_tasks = manifest
        .files
        .iter()
        .map(|file| {
            download_mod(
                file.project_id as u64,
                file.file_id as u64,
                directory.as_ref(),
            )
        })
        .collect::<Vec<_>>();

    // Await all tasks to complete
    future::join_all(download_tasks).await;

    Ok(())
}

async fn download_mod(
    project_id: u64,
    file_id: u64,
    directory: impl AsRef<Path>,
) -> Result<PathBuf, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", API_KEY.parse().unwrap());
    let request = client
        .get(format!(
            "https://api.curseforge.com/v1/mods/{}/files/{}",
            project_id, file_id
        ))
        .headers(headers);

    let response: Value = match request.send().await {
        Ok(response) => response.json().await.map_err(|err| {
            error!("Failed to parse JSON response: {}", err);
            "Failed to parse JSON response"
        })?,
        Err(err) => {
            error!("Request failed: {}", err);
            return Err("Request failed".into());
        }
    };

    let data = response.get("data").ok_or_else(|| {
        error!("Response does not contain 'data'");
        "Response does not contain 'data'"
    })?;

    let donwload_value = data.get("downloadUrl").ok_or_else(|| {
        error!("No 'downloadUrl' in response data");
        "No 'downloadUrl' in response data"
    })?;

    let download_url: String = if donwload_value.is_null() {
        get_no_api_download_url(
            file_id,
            data.get("fileName").unwrap().as_str().unwrap().to_string(),
        )?
    } else {
        donwload_value.as_str().unwrap().to_string()
    };

    let file_name = data
        .get("fileName")
        .ok_or_else(|| {
            error!("No 'fileName' in response data");
            "No 'fileName' in response data"
        })?
        .as_str()
        .ok_or_else(|| {
            error!("File name is not a valid string");
            "File name is not a valid string"
        })?
        .to_string();

    let file_path = directory.as_ref().join(Path::new(&file_name));

    info!(
        "Downloading {} from {} to {}",
        file_name,
        download_url,
        file_path.to_str().unwrap_or("[Invalid file path]")
    );

    let response = reqwest::get(download_url).await.map_err(|err| {
        error!("Failed to download file: {}", err);
        "Failed to download file"
    })?;

    let mut file = File::create(&file_path).map_err(|err| {
        error!("Failed to create file {}: {}", file_path.display(), err);
        "Failed to create file"
    })?;

    let bytes = response.bytes().await.map_err(|err| {
        error!("Failed to read response bytes: {}", err);
        "Failed to read response bytes"
    })?;

    file.write_all(&bytes).map_err(|err| {
        error!("Failed to write to file {}: {}", file_path.display(), err);
        "Failed to write to file"
    })?;

    Ok(file_path)
}

fn get_no_api_download_url(file_id: u64, file_name: String) -> Result<String, Box<dyn Error>> {
    let file_id = modify_id(file_id);
    Ok(format!(
        "https://mediafilez.forgecdn.net/files/{}/{}",
        file_id, file_name
    ))
}

fn modify_id(id: u64) -> String {
    let id_str = id.to_string();
    let first_part = &id_str[0..4];
    let remaining = id_str[4..].trim_start_matches('0').to_string();
    let remaining = if remaining.is_empty() {
        "0"
    } else {
        &remaining
    };
    format!("{}/{}", first_part, remaining)
}
