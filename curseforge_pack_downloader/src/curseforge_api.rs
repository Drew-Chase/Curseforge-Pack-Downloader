use crate::mod_file::ModFileResponse;
use crate::mod_type::{ModType, ModTypeExt};
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

pub struct ModDownloadProgressResponse {
    pub downloaded: u32,
    pub total: u32,
}

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

/// Downloads the latest version archive of a mod pack given a project ID.
///
/// This asynchronous function queries the CurseForge API for the latest
/// mod pack files associated with the specified project ID. It then
/// downloads the file and stores it in a temporary directory, returning
/// the path to the downloaded file.
///
/// # Arguments
///
/// * `project_id` - A `u64` representing the CurseForge project ID.
///
/// # Returns
///
/// A `Result` containing the `PathBuf` to the downloaded file if successful,
/// or a boxed error if any operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The API request fails.
/// - The API response does not contain the expected 'data' array.
/// - The 'downloadUrl' or 'fileName' fields are missing from the response.
/// - An error occurs during file creation or writing.
///
/// # Example
///
/// ```no-run
/// let path = download_latest_pack_archive(123456).await?;
/// println!("Downloaded to: {:?}", path);
/// ```
pub async fn download_latest_pack_archive(
    project_id: u64,
    temp_dir: impl AsRef<Path>,
) -> Result<PathBuf, Box<dyn Error>> {
    info!("Downloading the latest pack version");

    // Create a new HTTP client instance
    let client = Client::new();

    // Prepare the headers with the API key
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", header_parsed_api_key!());

    // Create the API request to get the files for the specified project ID
    let request = client
        .get(format!(
            "https://api.curseforge.com/v1/mods/{}/files",
            project_id
        ))
        .headers(headers);

    // Send the request and process the response
    let response: Value = match request.send().await {
        Ok(response) => response.json().await?, // Deserialize the response to JSON
        Err(err) => {
            // Return an error if the request fails
            return Err(Box::new(err));
        }
    };

    // Extract the 'data' field from the response JSON
    let data: Vec<Value> = response
        .get("data")
        .ok_or("Missing 'data' field in response")?
        .as_array()
        .ok_or("'data' field is not an array")?
        .to_vec();

    // Retrieve the first file in the 'data' array as the latest file
    let latest_file: Value = data
        .first()
        .ok_or("No files found in 'data' array")?
        .clone();

    // Extract the download URL and file name from the latest file
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
    
    create_dir_all(temp_dir.as_ref())?;

    // Determine the path to save the downloaded file
    let file_path = temp_dir.as_ref().join(file_name);

    // Create the file and write the downloaded content to it
    let mut file = File::create(&file_path)?;
    let response = reqwest::get(download_url).await?;
    let bytes = response.bytes().await?;
    file.write_all(&bytes)?;

    // Return the path to the downloaded file
    Ok(file_path)
}

/// Downloads mods specified in the manifest to the given directory.
///
/// # Arguments
///
/// * `manifest` - A reference to the manifest containing information about which mods to download.
/// * `directory` - The directory path where mods will be stored.
/// * `parallel` - The number of downloads to perform in parallel.
/// * `validate` - A flag indicating whether to validate downloaded files.
/// * `validate_if_size_less_than` - Optional size parameter. Files smaller than this value will be validated.
///
/// # Returns
///
/// A Result that is Ok if successful, or an error if any download or IO operation fails.
pub async fn download_mods_from_manifest<F>(
    manifest: &Manifest,
    directory: impl AsRef<Path>,
    parallel: u8,
    validate: bool,
    validate_if_size_less_than: Option<u64>,
    mut on_progress: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(ModDownloadProgressResponse) + 'static + Send + Sync,
{
    info!("Downloading mods from manifest");

    let directory = directory.as_ref().join("overrides");

    // Create the directory if it does not exist
    create_dir_all(&directory)?;

    // Determine the number of file chunks based on the parallel parameter
    let file_chunks =
        manifest
            .files
            .chunks(if parallel == 0 || parallel > manifest.files.len() as u8 {
                manifest.files.len()
            } else {
                parallel as usize
            });

    let total_mods_count = manifest.files.len() as u32;
    let mut mods_downloaded_count = 0u32;

    // Download each chunk of files
    for file_chunk in file_chunks {
        // Map each file in the chunk to a download task
        let download_tasks = file_chunk
            .iter()
            .map(|file| async {
                match download_mod(
                    file.project_id as u64,
                    file.file_id as u64,
                    directory.clone(),
                    validate,
                    validate_if_size_less_than,
                )
                .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Failed to download mod: {}", err);
                    }
                }
            })
            .collect::<Vec<_>>();

        warn!("Waiting for downloads to complete...");
        // Wait for all download tasks to complete before continuing
        future::join_all(download_tasks).await;
        mods_downloaded_count += file_chunk.len() as u32;
        on_progress(ModDownloadProgressResponse {
            downloaded: mods_downloaded_count,
            total: total_mods_count,
        });
    }

    Ok(())
}

/// Downloads a single mod file and writes it to the specified directory.
///
/// # Arguments
///
/// * `project_id` - The ID of the project to download.
/// * `file_id` - The ID of the file within the project.
/// * `directory` - The directory path where the file will be stored.
/// * `validate` - A flag indicating whether to validate the downloaded file.
/// * `validate_if_size_less_than` - Optional size parameter. Files smaller than this value will be validated.
///
/// # Returns
///
/// A Result containing the path to the downloaded file if successful, or an error if the download or any IO operation fails.
async fn download_mod(
    project_id: u64,
    file_id: u64,
    directory: impl AsRef<Path>,
    validate: bool,
    validate_if_size_less_than: Option<u64>,
) -> Result<PathBuf, Box<dyn Error>> {
    // Set a default value for validate_if_size_less_than if none is provided
    let validate_if_size_less_than = validate_if_size_less_than.unwrap_or(0);

    let client = Client::new();

    // Fetch project and file information from the API
    let project = get_project(project_id, &client).await?.data;
    let file_item = get_mod_item(project_id, file_id, &client).await?.data;

    // Determine if access to the download URL is denied
    let denied_api_access = file_item.download_url.is_none();
    let file_name = file_item.file_name.clone();

    // Obtain the download URL or a fallback URL if access is denied
    let download_url: String = if denied_api_access {
        get_no_api_download_url(file_id, &file_name)?
    } else {
        file_item.download_url.clone().ok_or_else(|| {
            error!("No 'downloadUrl' in response data");
            "No 'downloadUrl' in response data"
        })?
    };

    let directory = directory.as_ref();

    // Construct the full directory path based on the project class ID
    let directory = directory.join(project.class_id.unwrap_or(ModType::Mod).to_path());
    create_dir_all(&directory).map_err(|err| {
        error!(
            "Failed to create directory {}: {}",
            directory.display(),
            err
        );
        "Failed to create directory"
    })?;

    // Determine the final file path for the downloaded file
    let file_path = directory.join(Path::new(&file_name));
    info!(
        "Downloading {} from {} to {}",
        file_name,
        download_url,
        file_path.to_str().unwrap_or("[Invalid file path]")
    );

    // Perform the file download
    let response = reqwest::get(download_url).await.map_err(|err| {
        error!("Failed to download file: {}", err);
        "Failed to download file"
    })?;

    // Create the file where the downloaded bytes will be saved
    let mut file = File::create(&file_path).map_err(|err| {
        error!("Failed to create file {}: {}", file_path.display(), err);
        "Failed to create file"
    })?;

    // Read the downloaded bytes into the file
    let bytes = response.bytes().await.map_err(|err| {
        error!("Failed to read response bytes: {}", err);
        "Failed to read response bytes"
    })?;

    file.write_all(&bytes).map_err(|err| {
        error!("Failed to write to file {}: {}", file_path.display(), err);
        "Failed to write to file"
    })?;

    // Validate the file if needed and if it is below the size threshold
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

/// Generates a download URL using a file ID and name when API access is denied.
///
/// # Arguments
///
/// * `file_id` - A 64-bit identifier for the file.
/// * `file_name` - The name of the file to be downloaded.
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - A URL string on success or an error on failure.
fn get_no_api_download_url(
    file_id: u64,
    file_name: impl AsRef<str>,
) -> Result<String, Box<dyn Error>> {
    let file_name = file_name.as_ref();

    // Log a warning about using the no-api download URL
    warn!(
        "API access denied for '{}', falling back to no-api download url",
        file_name
    );

    // Encode the file name for URL compatibility
    let file_name = encode_uri_component(file_name);

    // Modify the file ID for URL path structure
    let file_id = modify_id(file_id)?;

    // Construct and return the final download URL
    Ok(format!(
        "https://mediafilez.forgecdn.net/files/{}/{}",
        file_id, file_name
    ))
}

/// Modifies a numeric ID for creating a part of a URL path.
///
/// # Arguments
///
/// * `id` - A 64-bit numeric identifier.
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - A formatted string on success or an error if the ID is invalid.
fn modify_id(id: u64) -> Result<String, Box<dyn Error>> {
    let id_str = id.to_string();
    let first_part = &id_str[0..4];
    let remaining = id_str[4..].trim_start_matches('0').to_string();

    // Handle cases where no remaining part is valid
    let remaining = if remaining.is_empty() {
        Err("Invalid Id")
    } else {
        Ok(&remaining)
    }?;

    // Return formatted string with parts separated by a '/'
    Ok(format!("{}/{}", first_part, remaining))
}

/// Validates a file's integrity by comparing its hash with an expected hash.
///
/// # Arguments
///
/// * `file_path` - The file system path for the file to validate.
/// * `hash` - The expected hash string to compare against.
///
/// # Returns
///
/// * `Result<bool, io::Error>` - `true` if the calculated hash matches the expected hash, `false` otherwise.
fn validate_file(file_path: impl AsRef<Path>, hash: impl AsRef<str>) -> Result<bool, io::Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Md5::new();
    let mut buffer = [0u8; 4096];

    // Read the file in chunks and update the hasher
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Calculate the hash and compare with the expected hash
    let calculated_hash = bytes_to_hex_string(&hasher.finalize());
    Ok(calculated_hash == hash.as_ref())
}

/// Converts a byte slice to a hex string.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to be converted into a hex string.
///
/// # Returns
///
/// * `String` - Hexadecimal representation of the input bytes.
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    // Map each byte to a 2-digit hex string and collect into a single string
    Map::collect(bytes.iter().map(|byte| format!("{:02x}", byte)))
}

/// Asynchronously retrieves a project from the CurseForge API.
///
/// # Arguments
///
/// * `project_id` - The unique identifier of the project to retrieve.
/// * `client` - The HTTP client to use for sending requests.
///
/// # Returns
///
/// A `Result` containing the `ProjectItem` if the operation succeeds,
/// or an error boxed as `Box<dyn Error>` if it fails.
pub async fn get_project(project_id: u64, client: &Client) -> Result<ProjectItem, Box<dyn Error>> {
    // Create a new header map to store request headers.
    let mut headers: HeaderMap = HeaderMap::new();

    // Insert the API key into the headers.
    headers.insert("x-api-key", header_parsed_api_key!());

    // Prepare the GET request to the CurseForge API, inserting the appropriate project ID.
    let request = client
        .get(format!("https://api.curseforge.com/v1/mods/{}", project_id))
        .headers(headers);

    // Send the request asynchronously and wait for the response.
    let response = request.send().await?;
    // Parse the JSON response into a ProjectItem.
    let data: ProjectItem = response.json().await?;

    // Return the parsed project data.
    Ok(data)
}

/// Asynchronously retrieves a mod file item related to a project from the CurseForge API.
///
/// # Arguments
///
/// * `project_id` - The unique identifier of the project to which the mod file belongs.
/// * `file_id` - The unique identifier of the mod file to retrieve.
/// * `client` - The HTTP client to use for sending requests.
///
/// # Returns
///
/// A `Result` containing the `ModFileResponse` if the operation succeeds,
/// or an error boxed as `Box<dyn Error>` if it fails.
async fn get_mod_item(
    project_id: u64,
    file_id: u64,
    client: &Client,
) -> Result<ModFileResponse, Box<dyn Error>> {
    // Create a new header map to store request headers.
    let mut headers: HeaderMap = HeaderMap::new();

    // Insert the API key into the headers.
    headers.insert("x-api-key", header_parsed_api_key!());

    // Prepare the GET request to the CurseForge API, inserting the appropriate project and file ID.
    let request = client
        .get(format!(
            "https://api.curseforge.com/v1/mods/{}/files/{}",
            project_id, file_id
        ))
        .headers(headers);

    // Send the request asynchronously and wait for the response.
    let response = request.send().await?;
    let response_text = response.text().await?;
    // Attempt to parse the JSON response into a ModFileResponse.
    let data: ModFileResponse = serde_json::from_str(&response_text).map_err(|err| {
        error!("Failed to parse response: {}", err);
        "Failed to parse response"
    })?;

    // Return the parsed mod file data.
    Ok(data)
}
