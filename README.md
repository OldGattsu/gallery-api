# Gallery API

Простое REST API на Rust с использованием фреймворка Axum 0.8.6 для работы с Яндекс.Диском.

## 🔧 Настройка переменных окружения

### Способ 1: Файл .env (рекомендуется для разработки)

Создайте файл `.env` в корне проекта:

```bash
# Яндекс.Диск API настройки
YANDEX_DISK_TOKEN=your_yandex_disk_token_here
YANDEX_DISK_API_URL=https://cloud-api.yandex.net/v1/disk

# Настройки сервера
SERVER_PORT=3000
```

### Способ 2: Системные переменные окружения

```bash
# Установка переменных
export YANDEX_DISK_TOKEN="your_token_here"
export YANDEX_DISK_API_URL="https://cloud-api.yandex.net/v1/disk"
export SERVER_PORT="3000"

# Запуск
cargo run
```

### Способ 3: Временные переменные для одной команды

```bash
YANDEX_DISK_TOKEN="your_token" cargo run
```

## Возможности

- ✅ RESTful API эндпоинты
- ✅ JSON сериализация/десериализация
- ✅ CORS поддержка
- ✅ Логирование запросов
- ✅ Обработка ошибок
- ✅ Фильтрация данных

## Запуск

```bash
# Установка зависимостей
cargo build

# Запуск сервера
cargo run
```

Сервер будет доступен по адресу: `http://localhost:3000`

## API Эндпоинты

### Основные

- `GET /` - Приветственное сообщение
- `GET /health` - Проверка состояния сервера

### Пользователи

- `GET /users` - Получить всех пользователей
  - Параметры запроса:
    - `name` (опционально) - фильтр по имени пользователя
  - Пример: `GET /users?name=alice`

- `GET /users/{id}` - Получить пользователя по ID
  - Параметры пути:
    - `id` - ID пользователя

- `POST /users` - Создать нового пользователя
  - Тело запроса (JSON):
    ```json
    {
      "name": "Имя пользователя",
      "email": "email@example.com"
    }
    ```

## Примеры использования

### Получить всех пользователей
```bash
curl http://localhost:3000/users
```

### Получить пользователя по ID
```bash
curl http://localhost:3000/users/1
```

### Создать нового пользователя
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

### Фильтрация пользователей по имени
```bash
curl "http://localhost:3000/users?name=alice"
```

### Проверка состояния сервера
```bash
curl http://localhost:3000/health
```

## Структура проекта

```
gallery/
├── Cargo.toml          # Зависимости проекта
├── src/
│   └── main.rs         # Основной код приложения
└── README.md           # Документация
```

## Технологии

- **Rust** - Язык программирования
- **Axum 0.8.6** - Веб-фреймворк
- **Tokio** - Асинхронная среда выполнения
- **Serde** - Сериализация/десериализация
- **Tower** - Middleware и утилиты
- **Tracing** - Логирование

## Разработка

Для разработки рекомендуется:

1. Установить Rust: https://rustup.rs/
2. Клонировать репозиторий
3. Запустить `cargo run` для разработки
4. Использовать `cargo test` для запуска тестов (когда будут добавлены)

## Лицензия

MIT
