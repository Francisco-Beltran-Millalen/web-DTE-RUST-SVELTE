# Implementation Decisions — DTE Auto-Upload

*Phase 4 persistence document | Initialized: Stage 4-1 | 2026-03-03*

**Read this at the start of every Stage 4-2 or 4-3 session.**
**Update the Progress section after every completed use case.**

---

## Tech Stack (Reference)

See `docs/tech-stack.md` for full version table. Summary:

| Layer | Technology |
|-------|-----------|
| Backend language | Rust 1.78+ (stable) |
| Backend framework | Axum 0.8 |
| Async runtime | Tokio 1.x |
| Database (prototype) | SQLite 3 via SQLx 0.8 |
| JWT | jsonwebtoken 9.x |
| Password hashing | argon2 0.5 |
| SII encryption | aes-gcm 0.10 |
| Frontend framework | SvelteKit 2.x (SPA mode) |
| Frontend language | Svelte 5 (runes) + TypeScript 5.x |
| CSS framework | Plain CSS (custom properties + scoped Svelte styles) |
| Backend tests | axum-test 19.x + tokio::test |

**Version note:** The plan specified `axum 0.7` but the project uses `axum 0.8` (required by `axum-test 19`). No API changes affected the skeleton code.

---

## Architecture Pattern

**Ports & Adapters (Hexagonal Architecture)**

**Rule summary:**
- Domain entities: zero external dependencies
- Use cases: depend on port traits only, never on adapters
- Inbound adapters: parse input → call use case → serialize output (no logic)
- Outbound adapters: translate trait call → SQL (no logic)
- DTOs: never reach the use case layer
- All async port traits: `#[async_trait]` (required for `dyn Trait`)
- DI: constructor injection with `Arc<dyn Trait + Send + Sync>`, wired in `main.rs`

**See `prototype-code/ARCHITECTURE_AND_FLOW_DESCRIPTION.md` for the full explanation, diagrams, and examples.**

---

## Folder Mapping

```
prototype-code/
├── ARCHITECTURE_AND_FLOW_DESCRIPTION.md
├── backend/
│   ├── src/
│   │   ├── main.rs                    ← bootstrap (wires deps, starts server)
│   │   ├── lib.rs                     ← library root (pub mods for integration tests)
│   │   ├── domain/mod.rs              ← pure entities (no external deps)
│   │   ├── application/
│   │   │   ├── errors.rs              ← AppError enum + IntoResponse
│   │   │   ├── ports/in/mod.rs        ← input port traits (handlers call these)
│   │   │   ├── ports/out/mod.rs       ← output port traits (use cases call these)
│   │   │   └── use_cases/mod.rs       ← use case implementations
│   │   ├── adapters/
│   │   │   ├── in/health_handler.rs   ← GET /health
│   │   │   ├── in/routes.rs           ← Router + AppState
│   │   │   ├── in/dto/mod.rs          ← request/response DTOs
│   │   │   └── out/mod.rs             ← SQLite repository implementations
│   │   └── config/
│   │       ├── app_config.rs          ← env vars (dotenvy)
│   │       └── database.rs            ← SqlitePool + FK pragma
│   ├── migrations/0001_initial.sql    ← full schema from docs/assets/schema.sql
│   ├── tests/health_test.rs           ← integration test (axum-test)
│   └── .env.example
└── frontend/
    ├── src/
    │   ├── app.css                    ← CSS custom properties (light + Everforest dark)
    │   ├── routes/+layout.ts          ← ssr=false, prerender=false
    │   ├── routes/+layout.svelte      ← root layout (imports app.css, init theme)
    │   ├── routes/(app)/
    │   │   ├── +layout.svelte         ← app shell: Sidebar + <main>
    │   │   └── +page.svelte           ← home page (DTE table + detail panel)
    │   ├── routes/(auth)/
    │   │   ├── +layout.svelte         ← centered layout + logo
    │   │   ├── login/+page.svelte
    │   │   └── register/+page.svelte
    │   └── lib/
    │       ├── api/client.ts          ← fetch wrapper (JWT + 401 retry)
    │       ├── stores/auth.svelte.ts  ← Svelte 5 $state auth store
    │       ├── stores/theme.svelte.ts ← Svelte 5 $state theme store (light/dark)
    │       ├── types/api.ts           ← shared TS interfaces
    │       └── components/
    │           ├── ThemeToggle.svelte ← sun/moon toggle button
    │           └── layout/
    │               └── Sidebar.svelte ← fixed 64px sidebar with active-route detection
    └── svelte.config.js               ← adapter-static + fallback: '200.html'
```

---

## File Naming Convention

| What | Pattern | Example |
|------|---------|---------|
| Domain entity | `{entity}.rs` | `dte.rs` |
| Input port trait | `{action}_{entity}_use_case.rs` | `get_dtes_use_case.rs` |
| Use case implementation | `{action}_{entity}_use_case_impl.rs` | `get_dtes_use_case_impl.rs` |
| Output port trait | `{entity}_repository.rs` | `dte_repository.rs` |
| Repository implementation | `{entity}_repository_impl.rs` | `dte_repository_impl.rs` |
| HTTP handler | `{entity}_handler.rs` | `dte_handler.rs` |
| DTO | `{entity}_{direction}_dto.rs` | `dte_response_dto.rs` |

---

## Stage 4-2 Approach

**End-to-end per use case.** Each session delivers:
1. Backend: domain entity → output port trait → use case impl → input port trait → handler → route registered
2. Frontend: API endpoint function → page/component → wired to store if needed
3. Verify: feature visible and working in the browser before moving to next use case

### Frontend Pages Sprint (before UC2)

Before continuing the backend use case sequence, three frontend pages were built to provide a real UI for testing:

| # | Route | File | Status |
|---|-------|------|--------|
| F1 | `/register` | `src/routes/register/+page.svelte` | ✓ done — wired to `POST /auth/register` (UC1) |
| F2 | `/login` | `src/routes/login/+page.svelte` | ✓ done — UI only, wired in UC2 |
| F3 | `/` | `src/routes/+page.svelte` | ✓ done — full Phase 3 design, mock data |

**Architecture note:** The home page (`+page.svelte`) includes the sidebar inline (no route group). When more app pages are added in later use cases, refactor to `routes/(app)/+layout.svelte` — this is tracked in Stage 4-4.

---

## Implementation Roadmap

Ordered by dependency. Each use case in Stage 4-2 delivers backend + frontend together.

| # | Use Case | Status |
|---|----------|--------|
| 1 | User can create an account | ✓ done |
| 2 | User can sign in | pending |
| 3 | User can sign out | pending |
| 4 | User can store their SII credentials | pending |
| 5 | User can update their SII credentials | pending |
| 6 | User can remove their SII credential | pending |
| 7 | User can connect their MercadoLibre account | pending |
| 8 | User can view their connected shops | pending |
| 9 | User can disconnect their MercadoLibre account | pending |
| 10 | User can reconnect a disconnected or errored shop | pending |
| 11 | System receives sale notifications (mock) | pending |
| 12 | System stores sales data | pending |
| 13 | System generates a DTE from sale data | pending |
| 14 | User can see all DTEs pending to upload | pending |
| 15 | User can view their DTE upload history | pending |
| 16 | User can view details of a specific DTE | pending |
| 17 | User can filter DTEs (status / shop / tipo / date) | pending |
| 18 | User can change the display order of DTEs | pending |
| 19 | User can upload a single DTE to SII (simulated) | pending |
| 20 | User can retry a failed DTE upload | pending |
| 21 | User can manually trigger a batch upload | pending |
| 22 | User can mark a DTE as manually uploaded | pending |
| 23 | User can download a DTE document (PDF or XML) | pending |
| 24 | User can view their notifications | pending |
| 25 | User can filter notifications / mark all as read | pending |
| 26 | User can change their password | pending |
| 27 | User can reset their password via email | pending |
| 28 | User can update their email address | pending |
| 29 | User can update their profile (name, phone, bio, social links) | pending |
| 30 | User can upload a profile photo | pending |
| 31 | User can delete their account | pending |

---

## Progress

### Stage 4-1 — Project Setup (2026-03-03) ✓

**Architecture approved:** Ports & Adapters (Hexagonal)

**Backend skeleton:**
- Cargo.toml with all dependencies (axum 0.8, sqlx 0.8, axum-test 19, etc.)
- Full folder structure established
- `GET /health` → 200 `{"status":"ok","database":"connected"}`
- `config/database.rs` — SqlitePool with `PRAGMA foreign_keys = ON` on every connection
- `config/app_config.rs` — all env vars loaded from dotenvy
- `migrations/0001_initial.sql` — full schema (19 tables + indexes)
- Unit test: `application::use_cases::tests::sanity_check` — passes
- Integration test: `health_returns_200_with_database_connected` — passes
- `cargo build` — zero errors, zero warnings

**Frontend skeleton:**
- SvelteKit 2.x + Svelte 5 (runes) + TypeScript
- `adapter-static` with `fallback: '200.html'` (SPA mode)
- `+layout.ts` — `ssr = false`, `prerender = false`
- Tailwind CSS 3 + PostCSS configured
- `lib/api/client.ts` — fetch wrapper with JWT and 401 retry stubs
- `lib/stores/auth.svelte.ts` — Svelte 5 `$state` auth store shell
- `lib/types/api.ts` — shared TypeScript interfaces (empty, populated per use case)
- `npm run build` — successful

**Exit criteria met:**
- [x] Architecture pattern chosen and approved
- [x] Architectural rules defined and documented
- [x] Folder structure approved
- [x] Implementation roadmap approved
- [x] Backend compiles and runs
- [x] SQLite connected with schema + FK enforcement
- [x] GET /health returns successfully
- [x] At least one unit test passes
- [x] At least one integration test passes
- [x] `consolidation-artifacts/implementation-decisions.md` initialized
- [x] User has run the project locally
- [x] `/export-log 4-1` run at the end

### Stage 4-3 — Learning Guide

*(UC1 completed under this mode. Switched to Stage 4-2b (Design-First) from UC2 onward.)*

#### UC1 — User can create an account ✓ (2026-03-13)

**Implemented:** Domain `User` entity, `UserRepository` output port, `CreateAccountUseCase` input port, `CreateAccountUseCaseImpl`, `UserRepositoryImpl` (SQLite), `create_account_handler`, `CreateAccountRequestDto`, `UserResponseDto`, route `POST /auth/register`.

**Decisions:**
- `User` fields are `pub` (required by `sqlx::query_as!` macro)
- Timestamp columns stored as `TEXT` with ISO8601 format (`strftime('%Y-%m-%dT%H:%M:%SZ', 'now')`) — required for SQLx chrono feature to work with SQLite
- `query_as!` uses explicit type overrides (`created_at as "created_at!: chrono::DateTime<chrono::Utc>"`) because SQLite declares columns as TEXT
- `last_insert_rowid()` used from Rust-side execute result (not SQL function) — avoids connection pool race condition
- `.sqlx/` offline cache committed — enables compile without live database

**Verified:** `POST /auth/register` returns 201 + `{id, name, email}`. Duplicate email returns 409 `{"error":"El email está en uso"}`.

**Next session:** UC2 — User can sign in

---

### Stage 4-2b — Design-First (in progress)

*(Starting from UC2. User designs modules + contracts; AI writes to spec.)*

#### Pre-UC2 Refactor: Folder Restructure (2026-03-20)

**Motivation:** Original structure used layered (`application/`, `adapters/in/`, `adapters/out/`) naming. User wanted structure that reflects hexagonal architecture concepts directly.

**New folder structure:**
```
src/
├── domain/
│   ├── entities/           ← pure business objects
│   ├── errors.rs           ← DomainError (business violations only)
│   ├── ports/
│   │   ├── inbound/        ← use case traits
│   │   └── outbound/       ← repository + service traits
│   └── services/           ← use case implementations
├── adapters/
│   ├── inbound/
│   │   ├── errors.rs       ← AppError: IntoResponse + From<DomainError>
│   │   ├── middleware/     ← JWT validation (populated in UC2)
│   │   ├── routes.rs
│   │   ├── handlers/
│   │   └── data_transfer_objects/
│   └── outbound/
│       ├── errors.rs       ← DbError: logs raw error + converts to DomainError
│       └── repositories/
├── config.rs               ← flat file: AppConfig + create_pool (merged)
├── lib.rs
└── main.rs
```

**Error flow decided:**
- `DbError` in `adapters/outbound/errors.rs` logs the raw SQLx error, returns `DomainError::Internal`
- `DomainError` travels through the use case unchanged
- `AppError` in `adapters/inbound/errors.rs` maps `DomainError` variants to HTTP status codes
- Inbound and outbound adapters never communicate directly

**JWT decided:**
- Token generation: outbound port (`TokenService`) + adapter implementation — use case calls the port
- Token validation: entirely in `adapters/inbound/middleware/` — runs before use case is called
- Domain receives `user_id` only; never sees a JWT string

**`cargo test` passed after refactor:** all 2 tests green.

---

## CSS Architecture (Stage 4-1 revisit — 2026-03-22)

**Tailwind removed.** Plain CSS with two layers:
1. `src/app.css` — CSS custom properties for all design tokens (light mode defaults + Everforest Medium Dark overrides via `[data-theme="dark"]` + `@media (prefers-color-scheme: dark)` guard)
2. Scoped `<style>` blocks per `.svelte` file — Svelte hashes classes at compile time

**`color-mix()` used for derived tokens** (`--bg-input`, `--bg-hover`, etc.) — re-evaluates when base token changes in dark mode, no duplication needed.

**Route groups:**
- `(app)/` — authenticated views with Sidebar layout
- `(auth)/` — unauthenticated views with centered layout + shared logo

**Theme toggle:** `themeStore` sets `document.documentElement.dataset.theme`. Init in root `+layout.svelte` `onMount`. Persisted in `localStorage`. ThemeToggle in Sidebar.

---

## Running Locally

**Backend:**
```bash
cd prototype-code/backend
cp .env.example .env       # fill in JWT_SECRET and SII_ENCRYPTION_KEY
cargo run                  # creates dev.db + applies migrations automatically, starts on http://localhost:3000
curl http://localhost:3000/health
```

**Frontend:**
```bash
cd prototype-code/frontend
npm install
npm run dev                # starts on http://localhost:5173
```

**Run tests:**
```bash
cd prototype-code/backend
cargo test                 # unit + integration tests
```
