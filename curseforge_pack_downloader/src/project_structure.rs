use crate::mod_type::ModType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileIndexItem {
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "fileId")]
    pub file_id: i64,
    pub filename: String,
    #[serde(rename = "releaseType")]
    pub release_type: i64,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModuleItem {
    pub name: String,
    pub fingerprint: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SortableGameVersionItem {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct HashItem {
    pub value: String,
    pub algo: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileItem {
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
    pub hashes: Vec<HashItem>,
    #[serde(rename = "fileDate")]
    pub file_date: String,
    #[serde(rename = "fileLength")]
    pub file_length: i64,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "sortableGameVersions")]
    pub sortable_game_versions: Vec<SortableGameVersionItem>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: i64,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: bool,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: i64,
    pub modules: Vec<ModuleItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Logo {
    pub id: i64,
    #[serde(rename = "modId")]
    pub mod_id: i64,
    pub title: String,
    pub description: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthorItem {
    pub id: i64,
    pub name: String,
    pub url: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CategoryItem {
    pub id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub name: String,
    pub slug: String,
    pub url: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "isClass")]
    pub is_class: bool,
    #[serde(rename = "classId")]
    pub class_id: i64,
    #[serde(rename = "parentCategoryId")]
    pub parent_category_id: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Links {
    #[serde(rename = "websiteUrl")]
    pub website_url: String,
    #[serde(rename = "wikiUrl")]
    pub wiki_url: String,
    #[serde(rename = "issuesUrl")]
    pub issues_url: String,
    #[serde(rename = "sourceUrl")]
    pub source_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub name: String,
    pub slug: String,
    pub links: Links,
    pub summary: String,
    pub status: i64,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    #[serde(rename = "isFeatured")]
    pub is_featured: bool,
    #[serde(rename = "primaryCategoryId")]
    pub primary_category_id: i64,
    pub categories: Vec<CategoryItem>,
    #[serde(rename = "classId")]
    pub class_id: ModType,
    pub authors: Vec<AuthorItem>,
    pub logo: Logo,
    pub screenshots: Vec<Logo>,
    #[serde(rename = "mainFileId")]
    pub main_file_id: i64,
    #[serde(rename = "latestFiles")]
    pub latest_files: Vec<FileItem>,
    #[serde(rename = "latestFilesIndexes")]
    pub latest_files_indexes: Vec<FileIndexItem>,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "dateReleased")]
    pub date_released: String,
    #[serde(rename = "allowModDistribution")]
    pub allow_mod_distribution: bool,
    #[serde(rename = "gamePopularityRank")]
    pub game_popularity_rank: i64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "hasCommentsEnabled")]
    pub has_comments_enabled: bool,
    #[serde(rename = "thumbsUpCount")]
    pub thumbs_up_count: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectItem {
    pub data: ProjectData,
}
