use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileModule {
    pub name: String,
    pub fingerprint: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SortableGameItem {
    #[serde(rename = "gameVersionName")]
    pub game_version_name: String,
    #[serde(rename = "gameVersionPadded")]
    pub game_version_padded: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "gameVersionReleaseDate")]
    pub game_version_release_date: String,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FileHashItem {
    pub value: String,
    pub algo: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ModFileItem {
    pub id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "modId")]
    pub mod_id: i64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "releaseType")]
    pub release_type: i64,
    #[serde(rename = "fileStatus")]
    pub file_status: i64,
    pub hashes: Vec<FileHashItem>,
    #[serde(rename = "fileDate")]
    pub file_date: String,
    #[serde(rename = "fileLength")]
    pub file_length: i64,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    #[serde(rename = "fileSizeOnDisk")]
    pub file_size_on_disk: i64,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "sortableGameVersions")]
    pub sortable_game_versions: Vec<SortableGameItem>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: i64,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: bool,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ModFiles {
    pub data: Vec<ModFileItem>,
}

#[derive(Serialize, Deserialize)]
pub struct ModFileResponse {
    pub data: ModFileItem,
}
