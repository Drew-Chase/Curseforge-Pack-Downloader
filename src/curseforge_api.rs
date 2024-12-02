use crate::pack_manifest::Manifest;
use futures::future;
use log::{error, info, warn};
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
    parallel: u8,
    validate: bool,
    validate_if_size_less_than: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    info!("Downloading mods from manifest");
    create_dir_all(&directory)?;
    // Downloads x at a time, where x is parallel
    let file_chunks =
        manifest
            .files
            .chunks(if parallel == 0 || parallel > manifest.files.len() as u8 {
                manifest.files.len()
            } else {
                parallel as usize
            });
    for file_chunk in file_chunks {
        let download_tasks = file_chunk
            .iter()
            .map(|file| {
                download_mod(
                    file.project_id as u64,
                    file.file_id as u64,
                    directory.as_ref(),
                    validate,
                    validate_if_size_less_than,
                )
            })
            .collect::<Vec<_>>();

        warn!("Waiting for downloads to complete...");

        // Await all tasks to complete
        future::join_all(download_tasks).await;
        info!("Downloads complete!");
    }

    Ok(())
}

async fn download_mod(
    project_id: u64,
    file_id: u64,
    directory: impl AsRef<Path>,
    validate: bool,
    validate_if_size_less_than: Option<u64>,
) -> Result<PathBuf, Box<dyn Error>> {
    let validate_if_size_less_than = validate_if_size_less_than.unwrap_or(0);
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

    let download_value = data.get("downloadUrl").ok_or_else(|| {
        error!("No 'downloadUrl' in response data");
        "No 'downloadUrl' in response data"
    })?;

    let denied_api_access = download_value.is_null();

    let download_url: String = if denied_api_access {
        get_no_api_download_url(
            file_id,
            data.get("fileName").unwrap().as_str().unwrap().to_string(),
        )?
    } else {
        download_value.as_str().unwrap().to_string()
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

    if validate && file_path.metadata()?.len() <= validate_if_size_less_than {
        if denied_api_access {
            error!(
                "Unable to validate file '{}', API access was denied!",
                file_name
            );
        } else {
            warn!("Validating {}...", file_name);
            let hashes = data
                .get("hashes")
                .ok_or_else(|| {
                    error!("No 'hashes' in response data");
                    "No 'hashes' in response data"
                })?
                .as_array()
                .ok_or_else(|| {
                    error!("'hashes' is not an array");
                    "'hashes' is not an array"
                })?;

            let md5_hash = hashes
                .iter()
                .find(|hash| hash.get("algo").unwrap().as_u64().unwrap() == 2)
                .ok_or_else(|| {
                    error!("No MD5 hash in response data");
                    "No MD5 hash in response data"
                })?
                .get("value")
                .ok_or_else(|| {
                    error!("No 'value' in MD5 hash");
                    "No 'value' in MD5 hash"
                })?
                .as_str()
                .ok_or_else(|| {
                    error!("MD5 hash is not a string");
                    "MD5 hash is not a string"
                })?;

            if !validate_file(&file_path, md5_hash)? {
                error!("File '{}' failed validation!", file_name);
                return Err("File failed validation".into());
            }
            info!("File '{}' passed validation!", file_name);
        }
    }

    Ok(file_path)
}

fn get_no_api_download_url(file_id: u64, file_name: String) -> Result<String, Box<dyn Error>> {
    warn!(
        "API access denied for '{}', falling back to no-api download url",
        file_name
    );
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

use md5::{Digest, Md5};
use std::io::{self, Read};

fn validate_file(file_path: impl AsRef<Path>, hash: impl AsRef<str>) -> Result<bool, io::Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Md5::new();
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let calculated_hash = bytes_to_hex_string(&hasher.finalize());
    Ok(calculated_hash == hash.as_ref())
}
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}
