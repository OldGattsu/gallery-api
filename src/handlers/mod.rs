use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use std::collections::HashMap;

use crate::{config::AppConfig, models::{YandexResource, YandexResourceQuery}};
use crate::models::{FileQuery, SimplifiedFile, SimplifiedFileList, YandexDiskInfo};
use crate::services;

// Обработчики
async fn root() -> &'static str {
    "Welcome to Gallery API!"
}


async fn health_check() -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();
    response.insert("status".to_string(), "healthy".to_string());
    response.insert("version".to_string(), "0.1.0".to_string());
    Json(response)
}

async fn get_files_list(
    Query(params): Query<FileQuery>,
    State(config): State<AppConfig>,
) -> Result<Json<SimplifiedFileList>, StatusCode> {
    // Получаем полный ответ от Яндекс.Диска
    let Json(full_response) = services::get_yandex_disk_files(params, config).await?;
    
    // Конвертируем в упрощенную структуру
    let simplified_items: Vec<SimplifiedFile> = full_response.items
        .into_iter()
        .map(|file| SimplifiedFile {
            name: file.name,
            path: file.path,
            size: file.size,
            preview: file.preview,
            created: file.created,
            modified: file.modified,
        })
        .collect();
    
    let total = simplified_items.len();
    
    let simplified_response = SimplifiedFileList {
        limit: full_response.limit,
        items: simplified_items,
        offset: full_response.offset,
        total,
    };

    tracing::info!("🔍 Simplified response: {:?}", simplified_response);
    
    Ok(Json(simplified_response))
}

async fn get_disk_info(State(config): State<AppConfig>) -> Result<Json<YandexDiskInfo>, StatusCode> {
    let Json(disk_info) = services::get_yandex_disk_info(config).await?;
    tracing::info!("🔍 Disk info: {:?}", disk_info);
    Ok(Json(disk_info))
}

async fn get_resource(
    Query(params): Query<YandexResourceQuery>,
    State(config): State<AppConfig>
) -> Result<Json<YandexResource>, StatusCode> {
    let Json(resource) = services::get_yandex_resource(params, config).await?;
    tracing::info!("🔍 Disk info: {:?}", resource);
    Ok(Json(resource))
}


pub fn create_router(config: AppConfig) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/files", get(get_files_list))
        .route("/resource", get(get_resource))
        .with_state(config)
}