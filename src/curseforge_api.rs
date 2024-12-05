use crate::mod_file::ModFileResponse;
use crate::mod_type::ModTypeExt;
use crate::pack_manifest::Manifest;
use crate::project_structure::ProjectItem;
use futures::future;
use log::{error, info, warn};
use md5::{Digest, Md5};
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::io::{self, Read};
use std::iter::Map;
use std::path::{Path, PathBuf};
use uri_encode::encode_uri_component;

macro_rules! header_parsed_api_key {
    () => {
        match std::env::var("CURSEFORGE_API_KEY").unwrap_or("".to_string()).parse(){
            Ok(key) => key,
            Err(err) => {
                error!("Failed to parse API key: {}", err);
                return Err("Failed to parse API key".into());
            }
        }
    };
}

pub async fn download_latest_pack_archive(project_id: u64) -> Result<PathBuf, Box<dyn Error>> {
    info!("Downloading the latest pack version");
    let client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", header_parsed_api_key!());

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

    let data: Vec<Value> = response.get("data")
                                   .ok_or("Missing 'data' field in response")?
        .as_array()
        .ok_or("'data' field is not an array")?
        .to_vec();
    let latest_file: Value = data.first()
                                 .ok_or("No files found in 'data' array")?
        .clone();
    let download_url: String = latest_file
        .get("downloadUrl")
        .ok_or("Missing 'downloadUrl' in latest file")?
        .as_str()
        .ok_or("'downloadUrl' is not a string")?
        .to_string();
    let file_name: String = latest_file
        .get("fileName")
        .ok_or("Missing 'fileName' in latest file")?
        .as_str()
        .ok_or("'fileName' is not a string")?
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
    println!("-> {:?}", directory.as_ref());
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
        info!("Downloading {} mods...", file_chunk.len());
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
    let client = Client::new();
    let project = get_project(project_id, &client).await?.data;
    let file_item = get_mod_item(project_id, file_id, &client).await?.data;

    let denied_api_access = file_item.download_url.is_none();

    let file_name = file_item.file_name.clone();
    let download_url: String = if denied_api_access {
        get_no_api_download_url(file_id, &file_name)?
    } else {
        file_item.download_url.clone().ok_or_else(|| {
            error!("No 'downloadUrl' in response data");
            "No 'downloadUrl' in response data"
        })?
    };

    let directory = directory.as_ref();

    let directory = directory.join(project.class_id.to_path());
    create_dir_all(&directory).map_err(|err| {
        error!(
            "Failed to create directory {}: {}",
            directory.display(),
            err
        );
        "Failed to create directory"
    })?;

    let file_path = directory.join(Path::new(&file_name));

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
            let md5_hash = file_item
                .hashes
                .iter()
                .find(|e| e.algo == 2)
                .ok_or_else(|| {
                    error!("No MD5 hash in response data");
                    "No MD5 hash in response data"
                })?;
            if !validate_file(&file_path, md5_hash.value.clone())? {
                error!("File '{}' failed validation!", file_name);
                return Err("File failed validation".into());
            }
            info!("File '{}' passed validation!", file_name);
        }
    }

    Ok(file_path)
}

fn get_no_api_download_url(
    file_id: u64,
    file_name: impl AsRef<str>,
) -> Result<String, Box<dyn Error>> {
    let file_name = file_name.as_ref();
    warn!(
        "API access denied for '{}', falling back to no-api download url",
        file_name
    );
    let file_name = encode_uri_component(file_name);
    let file_id = modify_id(file_id)?;
    Ok(format!(
        "https://mediafilez.forgecdn.net/files/{}/{}",
        file_id, file_name
    ))
}

fn modify_id(id: u64) -> Result<String, Box<dyn Error>> {
    let id_str = id.to_string();
    let first_part = &id_str[0..4];
    let remaining = id_str[4..].trim_start_matches('0').to_string();
    let remaining = if remaining.is_empty() {
        Err("Invalid Id")
    } else {
        Ok(&remaining)
    }?;
    Ok(format!("{}/{}", first_part, remaining))
}

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
    Map::collect(bytes.iter().map(|byte| format!("{:02x}", byte)))
}

pub async fn get_project(project_id: u64, client: &Client) -> Result<ProjectItem, Box<dyn Error>> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", header_parsed_api_key!());
    let request = client
        .get(format!("https://api.curseforge.com/v1/mods/{}", project_id))
        .headers(headers);
    let response = request.send().await?;
    let data: ProjectItem = response.json().await?;
    Ok(data)
}

async fn get_mod_item(
    project_id: u64,
    file_id: u64,
    client: &Client,
) -> Result<ModFileResponse, Box<dyn Error>> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key",header_parsed_api_key!());
    let request = client
        .get(format!(
            "https://api.curseforge.com/v1/mods/{}/{}",
            project_id, file_id
        ))
        .headers(headers);
    let response = request.send().await?;
    let data: ModFileResponse = response.json().await?;
    Ok(data)
}
