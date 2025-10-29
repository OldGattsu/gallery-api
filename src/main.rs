use gallery::{create_router, AppConfig};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Инициализация логирования
    tracing_subscriber::fmt::init();

    // Загружаем .env файл (если существует)
    if let Err(e) = dotenvy::dotenv() {
        tracing::info!("ℹ️  No .env file found or error loading it: {}", e);
        tracing::info!("   Using system environment variables only");
    } else {
        tracing::info!("✅ Loaded .env file successfully");
    }

    // Загружаем конфигурацию из переменных окружения
    let config = match AppConfig::from_env() {
        Ok(config) => {
            tracing::info!("✅ Configuration loaded successfully");
            tracing::info!("  YANDEX_DISK_API_URL: {}", config.yandex_disk_api_url);
            tracing::info!("  SERVER_PORT: {}", config.server_port);
            config
        }
        Err(e) => {
            tracing::error!("❌ Failed to load configuration: {}", e);
            tracing::error!("Please set YANDEX_DISK_TOKEN environment variable");
            std::process::exit(1);
        }
    };

    // Создание маршрутов с передачей конфигурации
    let app = create_router(config.clone()).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            ),
    );

    // Запуск сервера
    let bind_address = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("Failed to bind to address");

    tracing::info!("🚀 Server running on http://{}", bind_address);
    tracing::info!("📚 API Documentation:");
    tracing::info!("  GET  /           - Welcome message");
    tracing::info!("  GET  /health     - Health check");
    tracing::info!("  GET  /files      - Get files from Yandex Disk");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}