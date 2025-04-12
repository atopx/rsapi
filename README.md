# RSAPI - Rust API Framework

A high-performance API template built with Axum, designed for rapid development of secure and maintainable web services.

[![Rust](https://img.shields.io/badge/Language-Rust-green.svg?style=flat)](https://www.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/release/atopx/rsapi.svg)](https://github.com/atopx/rsapi/releases)
[![GitHub stars](https://img.shields.io/github/stars/atopx/rsapi)](https://github.com/atopx/rsapi/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/atopx/rsapi)](https://github.com/atopx/rsapi/network)
[![GitHub issues](https://img.shields.io/github/issues/atopx/rsapi)](https://github.com/atopx/rsapi/issues)

## Features

- 🚀 **Production-Ready**
  Fast and minimalistic web framework
- 🔐 **JWT Authentication**
  Secure token-based authentication system
- ⏰ **Cron Scheduler**
  Built-in task scheduling with `tokio-cron-scheduler`
- 📦 **ORM Integration**
  Database operations via SeaORM with PostgreSQL
- 📡 **Observability**
  Integrated tracing and structured logging
- 🛡️ **Security**
  CORS middleware and secure headers
- 🐳 **Minimal docker image**
  12.9MB Docker image with Alpine base

## Tech Stack

**Core Components**:
- Axum 0.6 (Web Framework)
- SeaORM 1.x (PostgreSQL ORM)
- JSON Web Tokens (JWT)
- Tokio 1.x (Async Runtime)

**Infrastructure**:

- Dockerized deployment
- Multi-stage build optimization
- Alpine Linux base image

## Quick Start

1. **Environment Setup**
   ```bash
   cp .env.sample .env
   # Update variables in .env file
   ```

2. **Database Setup**

Ensure PostgreSQL is running with credentials matching your [.env](./.env.sample) configuration.

3. Run with Docker

    ```bash
    docker-compose up --build -d
    ```


## API Examples

#### User Login :

```bash
curl -X POST http://localhost:6000/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"your_password"}'
```

#### Get User Claims :

```bash
curl -X GET http://localhost:6000/api/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Scheduled Tasks

```bash
CRONTAB_EXAMPLE="*/5 * * * * *"  # Every 5 seconds
```

Task implementations are in `[schedule::example](./src/schedule/mod.rs)` .

## Development

Build optimized binary:

```bash
cargo build --release
```

Run tests (ensure test database is configured):

```bash
cargo test -- --test-threads=1
```

---

License: [MIT](./LICENSE)
