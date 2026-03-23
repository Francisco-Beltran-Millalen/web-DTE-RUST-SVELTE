# DTE Auto-Upload

Full-stack web application for automating DTE (Documentos Tributarios Electrónicos) uploads to the Chilean SII (Servicio de Impuestos Internos).

## Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust · Axum 0.8 · SQLx 0.8 · SQLite |
| Frontend | SvelteKit · Svelte 5 · TypeScript · Tailwind CSS |
| Architecture | Hexagonal (Ports & Adapters) |
| Auth | JWT (access + refresh tokens) · Argon2 password hashing |

## Project Structure

```
├── backend/        Rust API server (Hexagonal Architecture)
│   ├── migrations/ SQL schema
│   └── src/
│       ├── domain/             Pure domain entities
│       ├── application/
│       │   ├── ports/in/       Input port traits (what handlers call)
│       │   ├── ports/out/      Output port traits (what use cases call)
│       │   └── use_cases/      Business logic implementations
│       ├── adapters/
│       │   ├── in/             HTTP handlers + DTOs (Axum)
│       │   └── out/            Repository implementations (SQLx)
│       └── config/             Environment + database setup
└── frontend/       SvelteKit SPA
    └── src/
        ├── lib/api/            REST client + typed endpoints
        ├── lib/stores/         Svelte 5 $state stores
        └── routes/             Page components
```

## Getting Started

### Prerequisites

- Rust (stable) — [rustup.rs](https://rustup.rs)
- Node.js 20+ and npm

### Backend

```bash
cd backend
cp .env.example .env
# Edit .env: generate JWT_SECRET and SII_ENCRYPTION_KEY with `openssl rand -hex 32`

cargo run
# API available at http://localhost:3000
```

Migrations run automatically on startup. No `sqlx-cli` required.

### Frontend

```bash
cd frontend
npm install
npm run dev
# App available at http://localhost:5173
```

### Running Tests

```bash
# Backend
cd backend && cargo test

# Frontend
cd frontend && npm run check
```

## Environment Variables

Copy `backend/.env.example` to `backend/.env` and fill in the values:

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | SQLite file path (e.g. `sqlite:./dev.db`) |
| `JWT_SECRET` | 32-byte hex secret (`openssl rand -hex 32`) |
| `JWT_ACCESS_TOKEN_EXPIRY_SECS` | Access token TTL in seconds |
| `JWT_REFRESH_TOKEN_EXPIRY_DAYS` | Refresh token TTL in days |
| `SII_ENCRYPTION_KEY` | 32-byte hex key for encrypting SII credentials |
| `PORT` | Server port (default: 3000) |
| `CORS_ALLOWED_ORIGINS` | Allowed CORS origin (e.g. `http://localhost:5173`) |

## Architecture

The backend follows **Hexagonal Architecture (Ports & Adapters)**:

- `domain/` — pure Rust structs with zero external dependencies
- `application/ports/` — trait contracts (no implementations)
- `application/use_cases/` — all business logic, depends only on port traits
- `adapters/in/` — Axum HTTP handlers, convert HTTP ↔ domain
- `adapters/out/` — SQLx repository implementations

See `ARCHITECTURE_AND_FLOW_DESCRIPTION.md` for a detailed walkthrough.
