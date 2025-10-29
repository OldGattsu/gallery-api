use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Яндекс.Диск API
#[derive(Deserialize)]
pub struct FileQuery {
    pub limit: Option<usize>,
    pub media_type: Option<String>,
    pub offset: Option<usize>,
    pub fields: Option<String>,
    pub preview_size: Option<String>,
    pub preview_crop: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YandexDiskFileSize {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YandexDiskFile {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub size: Option<u64>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub preview: Option<String>,
    pub media_type: Option<String>,
    pub mime_type: Option<String>,
    pub md5: Option<String>,
    pub sha256: Option<String>,
    pub sizes: Option<Vec<YandexDiskFileSize>>,
    pub resource_id: Option<String>,
    pub revision: Option<u64>,
    pub exif: Option<serde_json::Value>,
    pub antivirus_status: Option<String>,
    pub file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YandexDiskFileList {
    pub limit: u32,
    pub items: Vec<YandexDiskFile>,
    pub offset: u32,
}

// Упрощенная структура для ответа клиенту (только нужные поля)
#[derive(Serialize, Debug)]
pub struct SimplifiedFile {
    pub name: String,
    pub path: String,
    pub size: Option<u64>,
    pub preview: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SimplifiedFileList {
    pub limit: u32,
    pub items: Vec<SimplifiedFile>,
    pub offset: u32,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YandexDiskInfo {
    pub trash_size: u64,
    pub total_space: u64,
    pub used_space: u64,
    pub system_folders: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct YandexResourceQuery {
    pub path: Option<String>,
    pub limit: Option<usize>,
    
    #[serde(default = "default_fields")]
    pub fields: String,
}
fn default_fields() -> String {
    "name,modified, _embedded".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YandexResource {
    pub name: String,
    pub modified: String,
    pub _embedded: YandexResourceEmbedded,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YandexResourceEmbedded {
    pub path: String,
    pub limit: usize,
    pub offset: usize,
    pub sort: String,
    pub total: usize,
    pub items: Vec<YandexDiskFile>,
}