# Blockchain Node API

Этот проект представляет собой простую реализацию блокчейн-узла с использованием Rust и фреймворка Warp. API позволяет создавать узлы, добавлять блоки в блокчейн, получать информацию о блокчейне и синхронизировать узлы.

---

## Оглавление

- [Blockchain Node API](#blockchain-node-api)
  - [Оглавление](#оглавление)
  - [Требования](#требования)
  - [Установка и запуск](#установка-и-запуск)
  - [Примеры запросов](#примеры-запросов)

---

## Требования

Для запуска проекта вам понадобится:

- Установленный [Rust](https://www.rust-lang.org/tools/install).
- Утилита `curl` или любой другой инструмент для работы с HTTP-запросами (например, Postman).

---

## Установка и запуск

1. **Клонируйте репозиторий**:

    ```bash
    git clone https://github.com/OvchinnikovStepan/blockchain
    cd blockchain-node
    ```

2. **Собирите проект**:

    ```bash
    cargo build
    ```

3. **Запустите сервер**:

    По умолчанию сервер запускается на 127.0.0.1:3030. Вы можете указать порт и сложность Proof of Work через аргументы командной строки:

    ```bash
        cargo run -- --difficulty 4 --port 3030
    ```

    - --difficulty: Сложность Proof of Work (по умолчанию 4).
    - --port: Порт, на котором будет запущен сервер (по умолчанию 3030).

## Примеры запросов

1. **Создание нового узла**

    ```bash
    curl -X POST http://127.0.0.1:3030/node/create
    ```

    **Ответ:**

    ```json
    {
    "success": true,
    "message": "Node created: 127.0.0.1:3030",
    "status_code": 201,
    "data": "127.0.0.1:3030"
    }
    ```

2. **Получение блокчейна узла**

    ```bash
    curl -X GET http://127.0.0.1:3030/node/127.0.0.1:3030/chain
    ```

    **Ответ:**

    ```json
    {
    "success": true,
    "message": "Blockchain received",
    "status_code": 200,
    "data": [
        {
        "index": 0,
        "timestamp": 1698765432,
        "data": "Genesis Block",
        "previous_hash": "0",
        "hash": "abc123..."
        }
    ]
    }
    ```

3. **Добавление блока в блокчейн узла**

    ```bash
    curl -X POST http://127.0.0.1:3030/node/127.0.0.1:3030/block \
    -H "Content-Type: application/json" \
    -d '{"data": "New Block Data"}'
    ```

    **Ответ:**

    ```json
    {
    "success": true,
    "message": "Block added",
    "status_code": 200,
    "data": [
        {
        "index": 0,
        "timestamp": 1698765432,
        "data": "Genesis Block",
        "previous_hash": "0",
        "hash": "abc123..."
        },
        {
        "index": 1,
        "timestamp": 1698765433,
        "data": "New Block Data",
        "previous_hash": "abc123...",
        "hash": "def456..."
        }
    ]
    }
    ```

4. **Синхронизация блокчейна между узлами**

    ```bash
    curl -X POST http://127.0.0.1:3030/node/127.0.0.1:3030/sync/127.0.0.1:3031
    ```

    **Ответ:**

    ```json
    {
    "success": true,
    "message": "Chain synchronized",
    "status_code": 200,
    "data": [
        {
        "index": 0,
        "timestamp": 1698765432,
        "data": "Genesis Block",
        "previous_hash": "0",
        "hash": "abc123..."
        },
        {
        "index": 1,
        "timestamp": 1698765433,
        "data": "New Block Data",
        "previous_hash": "abc123...",
        "hash": "def456..."
        }
    ]
    }
    ```
