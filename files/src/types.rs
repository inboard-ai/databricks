use serde::{Deserialize, Serialize};

// ============================================================================
// DBFS types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct DbfsPut {
    pub path: String,
    pub contents: String, // base64-encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbfsReadResponse {
    #[serde(default)]
    pub bytes_read: i64,
    #[serde(default)]
    pub data: String, // base64-encoded
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileInfo {
    pub path: String,
    #[serde(default)]
    pub is_dir: bool,
    #[serde(default)]
    pub file_size: i64,
    #[serde(default)]
    pub modification_time: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbfsListResponse {
    #[serde(default)]
    pub files: Vec<FileInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DbfsMkdirs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DbfsMove {
    pub source_path: String,
    pub destination_path: String,
}

// ============================================================================
// Files API types (Unity Catalog volumes)
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct DirectoryEntry {
    pub path: String,
    #[serde(default)]
    pub is_directory: bool,
    #[serde(default)]
    pub file_size: Option<i64>,
    #[serde(default)]
    pub last_modified: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDirectoryResponse {
    #[serde(default)]
    pub contents: Vec<DirectoryEntry>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileStatus {
    pub path: String,
    #[serde(default)]
    pub is_directory: bool,
    #[serde(default)]
    pub file_size: Option<i64>,
    #[serde(default)]
    pub last_modified: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
