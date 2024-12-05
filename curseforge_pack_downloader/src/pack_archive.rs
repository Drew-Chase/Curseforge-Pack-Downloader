use crate::pack_manifest::Manifest;
use crate::{ProcessProgressResponse, ProcessStage};
use log::error;
use std::error::Error;
use std::fs;
use std::fs::remove_dir;
use std::path::{Path, PathBuf};

/// Processes a zip archive, extracts it, and downloads mods based on a manifest file.
///
/// This asynchronous function first extracts a specified zip archive to a temporary directory.
/// It verifies the presence of a manifest file within the extracted contents and subsequently
/// utilizes this manifest to manage the download of various mods. If any stage encounters an
/// error (like failing to extract, find the manifest, or download mods), the function logs the
/// error and will terminate the process.
///
/// # Parameters
/// - `zip_path`: A reference to the path where the zip archive file is located.
/// - `parallel`: The number of concurrent download operations for mods.
/// - `validate`: A boolean flag indicating whether the downloaded mods should be validated.
/// - `validate_if_size_less_than`: An optional size threshold (in bytes) under which files are validated.
/// - `temp_dir`: A reference to the path of the temporary directory where the archive is extracted.
///
/// # Returns
/// On success, returns a `Result` containing a tuple:
/// - `PathBuf`: The path to the extracted contents.
/// - `Manifest`: The manifest object created from the manifest file.
///
/// # Errors
/// - Will log an error and exit the process if the zip extraction fails or if the manifest file is not found.
/// - Will log an error, remove the mods directory, and exit the process if downloading mods fails.
pub async fn process_archive<F>(
    zip_path: impl AsRef<Path>,
    parallel: u8,
    validate: bool,
    validate_if_size_less_than: Option<u64>,
    temp_dir: impl AsRef<Path>,
    mut on_progress: F,
) -> Result<(PathBuf, Manifest), Box<dyn Error>>
where
    F: FnMut(ProcessProgressResponse) + 'static + Send + Sync,
{
    on_progress(ProcessProgressResponse {
        stage: ProcessStage::ExtractingArchive,
        progress: 0.1f32,
        message: "Extracting pack archive".to_string(),
    });

    // Attempt to extract the zip archive into a temporary directory.
    // The `extract_zip` function returns a `Result` containing
    // the path to the extracted files or an error if the extraction fails.
    let path = match extract_zip(zip_path, temp_dir) {
        // Successful extraction, store the resulting path.
        Ok(path) => path,
        // Log an error message and exit the process if extraction fails.
        Err(err) => {
            error!("Failed to extract zip archive: {}", err);
            std::process::exit(1);
        }
    };

    // Define the path to the manifest file within the extracted folder.
    let manifest_file = path.join("manifest.json");

    // Check for the existence of the manifest file.
    // Log an error and exit if the file is not found.
    if !manifest_file.exists() {
        error!("Manifest file not found!");
        std::process::exit(1);
    }

    // Attempt to create a new `Manifest` from the manifest file.
    // The `Manifest::new` function can return a `Result`.
    // The `?` operator is used to automatically handle the error scenario.
    let manifest = Manifest::new(manifest_file)?;

    // Attempt to download mods based on the information in the manifest.
    if let Err(e) = manifest
        .download_mods(
            &path,
            parallel,
            validate,
            validate_if_size_less_than,
            move |progress| {
                let mods_downloaded_percentage: f32 =
                    progress.downloaded as f32 / progress.total as f32;
                on_progress(ProcessProgressResponse {
                    stage: ProcessStage::DownloadingMods,
                    message: format!(
                        "Downloading {} of {} mods",
                        progress.downloaded, progress.total
                    ),
                    progress: (0.75f32 + mods_downloaded_percentage) / 1.2f32,
                })
            },
        )
        .await
    {
        // Attempt to remove the mods directory if downloading fails.
        if remove_dir(path).is_err() {
            error!("Failed to remove mods directory after failure");
        }
        // Log an error and terminate if mod downloads fail.
        error!("Failed to download mods: {}", e);
        std::process::exit(1);
    }

    // Return a successful result containing the path and manifest.
    Ok((path, manifest))
}

/// Extracts a zip file to a temporary directory.
/// Parameters:
/// - `zip_path`: The path to the zip file to be extracted.
/// - `temp_dir`: The directory where the contents of the zip will be extracted.
///
/// Returns a `Result` with the path to the temporary directory or an error.
fn extract_zip(
    zip_path: impl AsRef<Path>,
    temp_dir: impl AsRef<Path>,
) -> Result<PathBuf, Box<dyn Error>> {
    let temp_dir = temp_dir.as_ref();

    // Open the zip file.
    let zip_file = fs::File::open(zip_path)?;

    // Create a ZipArchive from the file.
    let mut zip_archive = zip::ZipArchive::new(zip_file)?;

    // Extract the contents of the zip archive into the temporary directory.
    zip_archive.extract(temp_dir.join(""))?;

    // Return the path to the temporary directory.
    Ok(temp_dir.to_path_buf())
}

/// Copies files from mod and override directories to an output directory.
/// Parameters:
/// - `overrides_dir`: The directory containing override files.
/// - `output_path`: The directory where files will be copied to.
///
/// Returns a `Result` with the path to the output directory or an error.
pub fn copy_to_output(
    overrides_dir: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<PathBuf, Box<dyn Error>> {
    // Ensure the output directory exists, creating it if necessary.
    fs::create_dir_all(&output_path)?;

    // Create a directory for mods inside the output directory.
    let new_mods = output_path.as_ref().join("mods");
    fs::create_dir_all(&new_mods)?;

    // Recursively copy the overrides directory, if it exists.
    if overrides_dir.as_ref().exists() {
        copy_dir_recursive(overrides_dir, &output_path)?;
    }

    // Return the path to the output directory.
    Ok(output_path.as_ref().to_path_buf())
}

/// Recursively copies a directory and its contents to another location.
/// Parameters:
/// - `src`: The source directory to copy from.
/// - `dest`: The destination directory to copy to.
///
/// Returns a `Result` indicating success or an I/O error.
fn copy_dir_recursive(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> std::io::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    if src.is_dir() {
        // Ensure the destination directory exists, creating it if necessary.
        fs::create_dir_all(dest)?;

        // Iterate over the entries in the source directory.
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            // Recursively copy subdirectories.
            if file_type.is_dir() {
                copy_dir_recursive(&src_path, &dest_path)?;
            } else {
                // Copy files from the source to the destination.
                fs::copy(&src_path, &dest_path)?;
            }
        }
    }

    // Return success.
    Ok(())
}
