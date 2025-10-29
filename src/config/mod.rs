use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub yandex_disk_token: String,
    pub yandex_disk_api_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        // Получаем токен Яндекс.Диска
        let yandex_disk_token = match env::var("YANDEX_DISK_TOKEN") {
            Ok(value) => value,
            Err(_) => return Err("YANDEX_DISK_TOKEN environment variable is required".to_string()),
        };

        // Получаем URL API (с значением по умолчанию)
        let yandex_disk_api_url = match env::var("YANDEX_DISK_API_URL") {
            Ok(value) => value,
            Err(_) => "https://cloud-api.yandex.net/v1/disk".to_string(),
        };

        // Получаем порт сервера (с значением по умолчанию)
        let server_port_str = match env::var("SERVER_PORT") {
            Ok(value) => value,
            Err(_) => "3000".to_string(),
        };

        // Парсим порт
        let server_port = match server_port_str.parse::<u16>() {
            Ok(port) => port,
            Err(_) => return Err("Invalid SERVER_PORT".to_string()),
        };

        Ok(AppConfig {
            yandex_disk_token,
            yandex_disk_api_url,
            server_port,
        })
    }
}
