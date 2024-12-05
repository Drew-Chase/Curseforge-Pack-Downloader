use crate::mod_type::ModType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileIndexItem {
	#[serde(rename = "gameVersion")]
	pub game_version: Option<String>,
	#[serde(rename = "fileId")]
	pub file_id: Option<i64>,
	pub filename: Option<String>,
	#[serde(rename = "releaseType")]
	pub release_type: Option<i64>,
	#[serde(rename = "gameVersionTypeId")]
	pub game_version_type_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModuleItem {
	pub name: Option<String>,
	pub fingerprint: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SortableGameVersionItem {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct HashItem {
	pub value: Option<String>,
	pub algo: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileItem {
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
	pub file_name: Option<String>,
	#[serde(rename = "releaseType")]
	pub release_type: Option<i64>,
	#[serde(rename = "fileStatus")]
	pub file_status: Option<i64>,
	pub hashes: Option<Vec<HashItem>>,
	#[serde(rename = "fileDate")]
	pub file_date: Option<String>,
	#[serde(rename = "fileLength")]
	pub file_length: Option<i64>,
	#[serde(rename = "downloadCount")]
	pub download_count: Option<i64>,
	#[serde(rename = "downloadUrl")]
	pub download_url: Option<Option<String>>,
	#[serde(rename = "gameVersions")]
	pub game_versions: Option<Vec<String>>,
	#[serde(rename = "sortableGameVersions")]
	pub sortable_game_versions: Option<Vec<SortableGameVersionItem>>,
	#[serde(rename = "alternateFileId")]
	pub alternate_file_id: Option<i64>,
	#[serde(rename = "isServerPack")]
	pub is_server_pack: Option<bool>,
	#[serde(rename = "fileFingerprint")]
	pub file_fingerprint: Option<i64>,
	pub modules: Option<Vec<ModuleItem>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Logo {
	pub id: Option<i64>,
	#[serde(rename = "modId")]
	pub mod_id: Option<i64>,
	pub title: Option<String>,
	pub description: Option<String>,
	#[serde(rename = "thumbnailUrl")]
	pub thumbnail_url: Option<String>,
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthorItem {
	pub id: Option<i64>,
	pub name: Option<String>,
	pub url: Option<String>,
	#[serde(rename = "avatarUrl")]
	pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CategoryItem {
	pub id: Option<i64>,
	#[serde(rename = "gameId")]
	pub game_id: Option<i64>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub url: Option<String>,
	#[serde(rename = "iconUrl")]
	pub icon_url: Option<String>,
	#[serde(rename = "dateModified")]
	pub date_modified: Option<String>,
	#[serde(rename = "isClass")]
	pub is_class: Option<bool>,
	#[serde(rename = "classId")]
	pub class_id: Option<i64>,
	#[serde(rename = "parentCategoryId")]
	pub parent_category_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Links {
	#[serde(rename = "websiteUrl")]
	pub website_url: Option<String>,
	#[serde(rename = "wikiUrl")]
	pub wiki_url: Option<String>,
	#[serde(rename = "issuesUrl")]
	pub issues_url: Option<String>,
	#[serde(rename = "sourceUrl")]
	pub source_url: Option<Option<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
	pub id: Option<i64>,
	#[serde(rename = "gameId")]
	pub game_id: Option<i64>,
	pub name: Option<String>,
	pub slug: Option<String>,
	pub links: Option<Links>,
	pub summary: Option<String>,
	pub status: Option<i64>,
	#[serde(rename = "downloadCount")]
	pub download_count: Option<i64>,
	#[serde(rename = "isFeatured")]
	pub is_featured: Option<bool>,
	#[serde(rename = "primaryCategoryId")]
	pub primary_category_id: Option<i64>,
	pub categories: Option<Vec<CategoryItem>>,
	#[serde(rename = "classId")]
	pub class_id: Option<ModType>,
	pub authors: Option<Vec<AuthorItem>>,
	pub logo: Option<Logo>,
	pub screenshots: Option<Vec<Logo>>,
	#[serde(rename = "mainFileId")]
	pub main_file_id: Option<i64>,
	#[serde(rename = "latestFiles")]
	pub latest_files: Option<Vec<FileItem>>,
	#[serde(rename = "latestFilesIndexes")]
	pub latest_files_indexes: Option<Vec<FileIndexItem>>,
	#[serde(rename = "dateCreated")]
	pub date_created: Option<String>,
	#[serde(rename = "dateModified")]
	pub date_modified: Option<String>,
	#[serde(rename = "dateReleased")]
	pub date_released: Option<String>,
	#[serde(rename = "allowModDistribution")]
	pub allow_mod_distribution: Option<bool>,
	#[serde(rename = "gamePopularityRank")]
	pub game_popularity_rank: Option<i64>,
	#[serde(rename = "isAvailable")]
	pub is_available: Option<bool>,
	#[serde(rename = "hasCommentsEnabled")]
	pub has_comments_enabled: Option<bool>,
	#[serde(rename = "thumbsUpCount")]
	pub thumbs_up_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectItem {
    pub data: ProjectData,
}
