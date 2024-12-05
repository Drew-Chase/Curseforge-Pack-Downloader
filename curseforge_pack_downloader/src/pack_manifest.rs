use crate::curseforge_api::ModDownloadProgressResponse;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct ModItem {
    #[serde(rename = "projectID")]
    pub project_id: i64,
    #[serde(rename = "fileID")]
    pub file_id: i64,
    pub required: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub version: Option<String>,
    pub author: Option<String>,
    pub files: Vec<ModItem>,
}

impl Manifest {
    /// Creates a new `Manifest` by reading it from a file at the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the file containing the manifest.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` if the manifest was successfully read and deserialized.
    /// * `Err` if there was an error opening the file or deserializing the contents.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        // Attempt to open the file located at the given path.
        // Return an error if the file cannot be opened.
        let file = std::fs::File::open(path)?;

        // Deserialize the contents of the file into a Manifest struct.
        // Return an error if deserialization fails.
        let manifest: Manifest = serde_json::from_reader(file)?;

        // Return the deserialized Manifest.
        Ok(manifest)
    }

    /// Downloads mods listed in the manifest to the specified directory.
    ///
    /// # Arguments
    ///
    /// * `directory` - The path to the directory where mods should be downloaded.
    /// * `parallel` - The number of downloads that can occur simultaneously.
    /// * `validate` - A boolean flag to indicate if mods should be validated after downloading.
    /// * `validate_if_size_less_than` - An optional size limit below which mods should be validated.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if mods were successfully downloaded.
    /// * `Err` if there was an error during the download or validation process.
    pub async fn download_mods<F>(
        &self,
        directory: impl AsRef<Path>,
        parallel: u8,
        validate: bool,
        validate_if_size_less_than: Option<u64>,
        on_progress: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(ModDownloadProgressResponse) + 'static + Send + Sync,
    {
        // Call the function to download mods based on the current manifest.
        // This operation is performed asynchronously.
        crate::curseforge_api::download_mods_from_manifest(
            self,
            directory,
            parallel,
            validate,
            validate_if_size_less_than,
            on_progress,
        )
        .await
    }
}
