use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileModule {
    pub name: Option<String>,
    pub fingerprint: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct SortableGameItem {
    #[serde(rename = "gameVersionName")]
    pub game_version_name: Option<String>,
    #[serde(rename = "gameVersionPadded")]
    pub game_version_padded: Option<String>,
    #[serde(rename = "gameVersion")]
    pub game_version: Option<String>,
    #[serde(rename = "gameVersionReleaseDate")]
    pub game_version_release_date: Option<String>,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FileHashItem {
    pub value: String,
    pub algo: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ModFileItem {
    pub id: Option<i64>,
    #[serde(rename = "gameId")]
    pub game_id: Option<i64>,
    #[serde(rename = "modId")]
    pub mod_id: Option<i64>,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<bool>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "releaseType")]
    pub release_type: Option<i64>,
    #[serde(rename = "fileStatus")]
    pub file_status: Option<i64>,
    pub hashes: Vec<FileHashItem>,
    #[serde(rename = "fileDate")]
    pub file_date: Option<String>,
    #[serde(rename = "fileLength")]
    pub file_length: Option<i64>,
    #[serde(rename = "downloadCount")]
    pub download_count: Option<i64>,
    #[serde(rename = "fileSizeOnDisk")]
    pub file_size_on_disk: Option<i64>,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "gameVersions")]
    pub game_versions: Option<Vec<String>>,
    #[serde(rename = "sortableGameVersions")]
    pub sortable_game_versions: Option<Vec<SortableGameItem>>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: Option<i64>,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: Option<bool>,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ModFiles {
    pub data: Vec<ModFileItem>,
}

#[derive(Serialize, Deserialize)]
pub struct ModFileResponse {
    pub data: ModFileItem,
}
