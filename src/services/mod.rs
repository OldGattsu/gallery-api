use crate::config::AppConfig;
use crate::models::{FileQuery, YandexDiskFileList, YandexDiskInfo, YandexResource, YandexResourceQuery};
use axum::http::StatusCode;
use axum::response::Json;

async fn request_wrapper<T>(url: &str, token: &str) -> Result<Json<T>, StatusCode>
where T: serde::de::DeserializeOwned
{
    let client = reqwest::Client::new();
    match client
        .get(url)
        .header("Authorization", format!("OAuth {}", token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = response.text().await.unwrap_or_else(|_| "Failed to get response text".to_string());
                tracing::debug!("📄 Raw response: {}", response_text);
                
                // Попробуем распарсить JSON
                match serde_json::from_str::<T>(&response_text) {
                    Ok(file_list) => {
                        Ok(Json(file_list))
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to parse response: {}", e);
                        tracing::debug!("📄 Response text was: {}", response_text);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                tracing::error!("❌ Yandex Disk API error: {}", error_text);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            tracing::error!("❌ Request failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_yandex_disk_files(
    params: FileQuery,
    config: AppConfig,
) -> Result<Json<YandexDiskFileList>, StatusCode> {
    // Параметры запроса с значениями по умолчанию
    let limit = params.limit.unwrap_or(20);
    let media_type = params.media_type.unwrap_or_else(|| "image".to_string());
    let offset = params.offset.unwrap_or(0);
    let fields = params.fields.unwrap_or_else(|| "_embedded.items.name,_embedded.items.path,_embedded.items.type,_embedded.items.size,_embedded.items.created,_embedded.items.modified,_embedded.items.preview,_embedded.items.media_type".to_string());
    let preview_size = params.preview_size.unwrap_or_else(|| "M".to_string());
    let preview_crop = params.preview_crop.unwrap_or(false);

    // Формируем URL согласно документации Яндекс.Диска
    let url = format!(
        "{}/disk/resources/files?limit={}&media_type={}&offset={}&fields={}&preview_size={}&preview_crop={}&path=japan_november",
        config.yandex_disk_api_url,
        limit,
        media_type,
        offset,
        fields,
        preview_size,
        preview_crop
    );
    
    tracing::info!("🔗 Requesting: {}", url);
    
    request_wrapper::<YandexDiskFileList>(&url, &config.yandex_disk_token).await
}


pub async fn get_yandex_disk_info(config: AppConfig) -> Result<Json<YandexDiskInfo>, StatusCode> {
    let url = format!("{}/disk", config.yandex_disk_api_url);

    tracing::info!("🔗 Requesting: {}", url);

    request_wrapper::<YandexDiskInfo>(&url, &config.yandex_disk_token).await
}

pub async fn get_yandex_resource(params: YandexResourceQuery, config: AppConfig) -> Result<Json<YandexResource>, StatusCode> {
    let limit = params.limit.unwrap_or(20);
    let path = params.path.unwrap_or_else(|| "".to_string());
    let fields = params.fields;

    let url = format!("{}/disk/resources?limit={}&path={}&fields={}", config.yandex_disk_api_url, limit, path, fields);

    tracing::info!("🔗 Requesting: {}", url);

    request_wrapper::<YandexResource>(&url, &config.yandex_disk_token).await
}