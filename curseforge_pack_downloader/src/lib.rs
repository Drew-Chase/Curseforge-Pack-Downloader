#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use crate::pack_manifest::Manifest;
use log::{error, info};
use std::error::Error;
use std::path::{Path, PathBuf};

mod curseforge_api;
mod mod_file;
mod mod_type;
mod pack_archive;
mod pack_manifest;
mod project_structure;

/// This module contains types and functionalities related to processing and downloading
/// Curseforge mod packs. It includes a structure `CurseforgePackDownloader` which
/// provides configurable options such as setting the output directory, parallel
/// downloads, and validation settings for downloaded mod files. The module also offers
/// methods to process mod pack archives, either by ID or file path, and returns the
/// resulting mod pack manifest upon successful processing.
pub struct CurseforgePackDownloader {
    /// Where the downloaded finalized pack will output.
    /// You can use %PACK_NAME% as a variable for the pack name.
    /// Ex: ./packs/%PACK_NAME% -> ./packs/All the Mods 10
    /// Here is a list of all possible variables
    /// - **PACK_NAME** *(the name of the modpack)*
    /// - **PACK_VERSION** *(the version of the modpack)*
    /// - **PACK_AUTHOR** *(the primary author of the modpack)*
    /// - **TIME** *(the current time in ms - this can be great for creating unique paths)*
    output_dir: PathBuf,
    /// Where the temporary files will be stored during processing
    /// This file will be removed once the program finishes
    temp_directory: PathBuf,

    /// This will validate the downloaded mods based on the provided hash. (Note: this can take significantly longer)
    validate: bool,

    /// The number of mods that will be downloaded in parallel
    /// This can allow for much faster downloading.
    /// Higher may not be better, due to internet speeds.
    parallel_downloads: u8,

    /// This will only attempt to validate files where the file size is less than this value (in bytes)
    validate_if_size_less_than: Option<u64>,
}

impl Default for CurseforgePackDownloader {
    fn default() -> Self {
        Self::new()
    }
}

impl CurseforgePackDownloader {
    /// Creates a new `CurseforgePackDownloader` instance with the specified API key.
    ///
    /// # Parameters
    ///
    /// - `curseforge_api_key`: A string slice representing the API key to authenticate requests
    ///   to the Curseforge API.
    pub fn new() -> Self {
        Self {
            output_dir: PathBuf::from("./"),
            temp_directory: PathBuf::from("./temp"),
            validate: false,
            parallel_downloads: 16,
            validate_if_size_less_than: None,
        }
    }

    /// Sets the temporary directory for storing files during processing.
    ///
    /// # Parameters
    ///
    /// - `temp_directory`: A reference to the path representing the temporary directory.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the current `CurseforgePackDownloader` instance.
    pub fn set_temp_directory(&mut self, temp_directory: impl AsRef<Path>) -> &mut Self {
        self.temp_directory = temp_directory.as_ref().to_path_buf();
        self
    }

    /// Sets the output directory where the finalized pack will be stored.
    /// You can use the following variables to customize the directory:
    /// - **%PACK_NAME%**: the name of the modpack
    /// - **%PACK_VERSION%**: the version of the modpack
    /// - **%PACK_AUTHOR%**: the primary author of the modpack
    /// - **%TIME%**: the current time in milliseconds (useful for creating unique paths)
    ///
    /// # Parameters
    ///
    /// - `output_directory`: A reference to the path representing the output directory.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the current `CurseforgePackDownloader` instance.
    pub fn set_output_directory(&mut self, output_directory: impl AsRef<Path>) -> &mut Self {
        self.output_dir = output_directory.as_ref().to_path_buf();
        self
    }

    /// Configures whether to validate the downloaded mods based on their hash.
    ///
    /// # Parameters
    ///
    /// - `validate`: A boolean determining if files should be validated.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the current `CurseforgePackDownloader` instance.
    pub fn set_validate(&mut self, validate: bool) -> &mut Self {
        self.validate = validate;
        self
    }

    /// Sets the number of downloads that can occur in parallel.
    ///
    /// # Parameters
    ///
    /// - `parallel_downloads`: The number of parallel downloads to perform.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the current `CurseforgePackDownloader` instance.
    pub fn set_parallel_downloads(&mut self, parallel_downloads: u8) -> &mut Self {
        self.parallel_downloads = parallel_downloads;
        self
    }

    /// Sets the file size limit for validation. Only files smaller than this size will be validated.
    ///
    /// # Parameters
    ///
    /// - `validate_if_size_less_than`: The size threshold in bytes.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the current `CurseforgePackDownloader` instance.
    pub fn set_validate_if_size_less_than(&mut self, validate_if_size_less_than: u64) -> &mut Self {
        self.validate_if_size_less_than = Some(validate_if_size_less_than);
        self
    }

    /// Downloads and processes the mod pack archive for the given mod pack ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the mod pack to process.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Manifest` of the processed pack or an error if
    /// processing fails.
    pub async fn process_id(&self, id: u64) -> Result<Manifest, Box<dyn Error>> {
        let file = curseforge_api::download_latest_pack_archive(id).await?;
        self.process_file(file).await
    }

    /// Processes a mod pack archive from the given file path.
    ///
    /// # Parameters
    ///
    /// - `file`: A reference to the path of the file to process.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Manifest` of the processed pack or an error if
    /// processing fails.
    pub async fn process_file(&self, file: impl AsRef<Path>) -> Result<Manifest, Box<dyn Error>> {
        // Initiate processing of the archive file
        // This function extracts the archive, validates the contents, and downloads needed mods
        let (tmp, manifest) = match pack_archive::process_archive(
            file,
            self.parallel_downloads,
            self.validate,
            self.validate_if_size_less_than,
            &self.temp_directory,
        ).await
        {
            // Continue if processing is successful, store results
            Ok(output) => output,
            // Return error if processing fails
            Err(err) => {
                return Err(err);
            }
        };

        // Parse the output directory path using the manifest data
        let output = self.get_parsed_output(&manifest);

        // Define paths for 'mods' and 'overrides' directories within temporary files
        let mods = tmp.join("mods");
        let overrides = tmp.join("overrides");

        // Copy contents from 'mods' and 'overrides' directories to the final output location
        match pack_archive::copy_to_output(mods, overrides, output) {
            // Log successful copy operation
            Ok(output) => {
                info!("Pack copied to {}", output.display());
            }
            // Log and return error if copying fails
            Err(err) => {
                error!("Unable to copy pack to output: {}", err);
                return Err(err);
            }
        };

        // Attempt to remove the temporary directory used during processing
        match std::fs::remove_dir_all(tmp) {
            // Log successful removal of the temporary directory
            Ok(_) => {
                info!("Temp directory removed");
            }
            // Log and return error if removal fails
            Err(err) => {
                error!("Unable to remove temp directory: {}", err);
                // Return detailed error message for failure to clean up
                return Err(format!("Unable to remove temp directory: {}", err).into());
            }
        }

        // Return the manifest object, indicating successful completion of the process
        Ok(manifest)
    }

    /// Parses the `output_dir` path by replacing placeholders with metadata from a specified manifest.
    ///
    /// It substitutes placeholders in the `output_dir`'s string representation
    /// with actual values from a `Manifest` object. Supported placeholders include:
    /// - `%PACK_NAME%`: The name of the package from the manifest.
    /// - `%PACK_VERSION%`: The version of the package from the manifest.
    /// - `%PACK_AUTHOR%`: The author of the package from the manifest.
    /// - `%TIME%`: The current time in milliseconds since the UNIX epoch.
    ///
    /// If the path cannot be converted to a string, an empty string is used, and an error is logged.
    /// In case retrieving the system time fails, a default of `0` milliseconds is used.
    ///
    /// # Parameters
    /// - `manifest`: The `Manifest` providing data for placeholder substitution.
    ///
    /// # Returns
    /// A `PathBuf` with placeholders in the `output_dir` replaced by their corresponding values from the manifest data.
    ///
    /// # Errors
    /// Errors are logged, and default values are used to handle them gracefully.
    fn get_parsed_output(&self, manifest: &Manifest) -> PathBuf {
        use log::error;
        use std::time::{Duration, SystemTime};

        // Convert the path to a string safely.
        // If the conversion fails, log an error and use an empty string as a fallback.
        let mut path_string: String = self
            .output_dir
            .to_str()
            .unwrap_or_else(|| {
                error!("Unable to convert path to string");
                ""
            })
            .to_string();

        // Extract metadata from the manifest for substitution into the path string.
        let pack_name = &manifest.name;
        let pack_version = &manifest.version;
        let author = &manifest.author;

        // Get the current system time and convert it to milliseconds since the UNIX epoch.
        // If this fails, log an error and use a default of 0 milliseconds.
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|err| {
                error!("Unable to get current time: {}", err);
                Duration::new(0, 0)
            })
            .as_millis();

        // Replace placeholders in the path string with actual values from the manifest.
        path_string = path_string.replace("%PACK_NAME%", pack_name.as_str());
        path_string = path_string.replace("%PACK_VERSION%", pack_version.as_str());
        path_string = path_string.replace("%PACK_AUTHOR%", author.as_str());
        path_string = path_string.replace("%TIME%", &time.to_string());

        // Convert the final string back to a PathBuf for use in the program.
        PathBuf::from(path_string)
    }
}
