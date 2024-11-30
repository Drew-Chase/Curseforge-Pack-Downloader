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
pub struct ModLoaderItem {
    pub id: String,
    pub primary: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Minecraft {
    pub version: String,
    #[serde(rename = "modLoaders")]
    pub mod_loaders: Vec<ModLoaderItem>,
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub minecraft: Minecraft,
    #[serde(rename = "manifestType")]
    pub manifest_type: String,
    #[serde(rename = "manifestVersion")]
    pub manifest_version: i64,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<ModItem>,
    pub overrides: String,
}

impl Manifest {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        let manifest: Manifest = serde_json::from_reader(file)?;
        Ok(manifest)
    }

    pub async fn download_mods(&self, directory: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        crate::curseforge_api::download_mods_from_manifest(self, directory).await
    }
}
