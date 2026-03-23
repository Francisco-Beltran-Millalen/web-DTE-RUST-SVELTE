# Architecture and Data Flow — DTE Auto-Upload

*Stage 4-1 | Written for anyone joining the project*

---

## What Problem Does Hexagonal Architecture Solve?

In a typical small project, everything talks to everything: the HTTP handler queries the database directly, the business logic is mixed with SQL strings, and testing requires a real database. As the project grows, this becomes painful:

- You can't test business logic without spinning up a database
- Swapping the database (e.g., SQLite → PostgreSQL) requires touching half the codebase
- A change in the API response format ripples into the business logic

**Hexagonal Architecture (Ports & Adapters)** solves this by establishing a strict rule:

> The business logic at the center of the application knows nothing about the outside world. It only talks to abstract contracts (traits/interfaces). Everything else — HTTP, databases, file systems — plugs into those contracts from the outside.

The result:
- Business logic is testable in isolation (no HTTP, no database)
- The database can be swapped without touching the use cases
- HTTP handlers can change format without touching business logic
- The shape of the system is obvious from the folder structure

---

## Backend Layer Map

```
prototype-code/backend/src/
│
├── adapters/in/          ← INBOUND ADAPTERS (HTTP layer)
│   ├── health_handler.rs    HTTP handler functions
│   ├── routes.rs            Route registration + AppState
│   └── dto/                 Request/response data shapes (JSON ↔ Rust)
│
├── application/
│   ├── ports/in/         ← INPUT PORT TRAITS (what handlers call)
│   ├── ports/out/        ← OUTPUT PORT TRAITS (what use cases call)
│   ├── use_cases/        ← USE CASE IMPLEMENTATIONS (all business logic)
│   └── errors.rs            Shared error types
│
├── adapters/out/         ← OUTBOUND ADAPTERS (database layer)
│   └── (entity)_repository_impl.rs   SQLite implementations
│
├── domain/               ← DOMAIN ENTITIES (pure data, no dependencies)
│   └── (entity).rs
│
└── config/               ← STARTUP / INFRASTRUCTURE
    ├── app_config.rs        Environment variables
    └── database.rs          SQLite pool setup
```

**What each layer is allowed to do:**

| Layer | Allowed | NOT Allowed |
|-------|---------|-------------|
| `domain/` | Define structs, enums, pure logic | Import from any other layer; do I/O |
| `application/ports/` | Define trait contracts | Import adapters; do I/O |
| `application/use_cases/` | Call port traits; implement business logic | Import adapters directly; call SQLx |
| `adapters/in/` | Handle HTTP; call input port traits | Contain business logic; call SQLx |
| `adapters/out/` | Implement output port traits; call SQLx | Contain business logic |
| `config/` | Bootstrap infrastructure | Contain business logic |

---

## Request Flow — Step by Step

**Example: User registers an account (`POST /auth/register`)**

```
Browser
  │  POST /auth/register
  │  { "name": "Juan", "email": "juan@...", "password": "secret" }
  │
  ▼
adapters/in/routes.rs           → Axum router receives the request
  │
  ▼
adapters/in/auth_handler.rs     → Handler extracts + validates the JSON body
  │                               (rejects obviously malformed input)
  │  calls trait method:
  ▼
application/ports/in/
  register_user_use_case.rs     → Input port trait: RegisterUserUseCase
  │                               (just a trait — no implementation here)
  │  implemented by:
  ▼
application/use_cases/
  register_user_use_case_impl.rs → Business logic:
  │                                 1. Check email not already taken
  │                                 2. Hash the password (argon2)
  │                                 3. Build domain entity
  │  calls trait method:
  ▼
application/ports/out/
  user_repository.rs             → Output port trait: UserRepository
  │                               (just a trait — no SQL here)
  │  implemented by:
  ▼
adapters/out/
  user_repository_impl.rs        → SQLx: INSERT INTO user (...)
  │
  ▼
SQLite database
  │
  (returns created user row)
  │
  ▼
use case assembles domain::User entity
  │
  ▼
adapters/in/dto/
  user_response_dto.rs           → Converts User entity → UserResponseDto
  │
  ▼
handler serializes DTO to JSON
  │
  ▼
HTTP Response 201 Created
  { "id": 1, "name": "Juan", "email": "juan@..." }
```

### Why so many layers for one registration?

Each layer has exactly one responsibility and can change independently:
- The JSON format can change (DTO layer) without touching business logic
- The password hashing algorithm can change (use case layer) without touching HTTP
- SQLite can be replaced with PostgreSQL (adapter/out layer) without touching anything else
- The business rule "email must be unique" lives in exactly one place

---

## Frontend Flow — Step by Step

**Example: User submits the registration form**

```
User fills out the form and clicks "Register"
  │
  ▼
+page.svelte (routes/auth/register/)
  │  on:submit handler runs
  │
  ▼
lib/api/endpoints/auth.ts
  │  register(name, email, password) function called
  │  builds the request body
  │
  ▼
lib/api/client.ts
  │  apiFetch('POST', '/auth/register', body)
  │  adds Authorization header if a token exists (not needed for register)
  │  handles 401 retry logic (not needed for register)
  │
  ▼
fetch() → REST API (http://localhost:3000/auth/register)
  │
  (backend handles the request — see above)
  │
  ▼
Response received
  │
  ▼
lib/api/endpoints/auth.ts
  │  parses JSON → AuthResponse typed interface (from lib/types/api.ts)
  │
  ▼
lib/stores/auth.svelte.ts
  │  authStore.setUser(user, token)
  │  Svelte 5 $state updates → all subscribers re-render
  │
  ▼
Router navigates to the home page
  │
  ▼
UI reflects logged-in state
```

### Svelte 5 Runes

Stores use `$state` (Svelte 5 runes) instead of Svelte 4's writable stores:

```ts
// Reactive state — any component reading authStore.user
// automatically re-renders when it changes.
let state = $state({ user: null, isAuthenticated: false });
```

No manual subscriptions, no `$` prefix in components. Runes handle reactivity automatically.

---

## Dependency Injection in Rust (No DI Container Needed)

Other languages use DI containers (Spring, NestJS) to wire up dependencies. Rust doesn't need one. The pattern is:

**Constructor injection with `Arc<dyn Trait + Send + Sync>`**

```rust
// Output port trait (contracts what the use case needs)
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn create(&self, user: NewUser) -> Result<User, AppError>;
}

// Use case holds a reference-counted pointer to any implementor
pub struct RegisterUserUseCaseImpl {
    user_repo: Arc<dyn UserRepository>,
}

// SQLite implementation (injected at startup)
pub struct SqliteUserRepository {
    pool: SqlitePool,
}
#[async_trait]
impl UserRepository for SqliteUserRepository { ... }
```

**Wired in main.rs (once, at startup):**

```rust
let user_repo = Arc::new(SqliteUserRepository::new(pool.clone()));
let register_uc = Arc::new(RegisterUserUseCaseImpl::new(user_repo.clone()));

let state = AppState {
    register_user: register_uc,
    // ...other use cases
};
```

**AppState grows as use cases are added:**

```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    // Added per use case in Stage 4-2:
    // pub register_user: Arc<dyn RegisterUserUseCase>,
    // pub sign_in: Arc<dyn SignInUseCase>,
}
```

`Arc` (atomic reference count) makes the state safely cloneable and shareable across Tokio's async tasks. `dyn Trait + Send + Sync` allows any implementation to be swapped in — including a mock in tests.

---

## Architectural Rules (Non-Negotiable)

These rules exist to keep the architecture clean as use cases are added:

1. **Domain entities have zero external dependencies.** No SQLx, no Axum, no Serde unless it's a derive macro. Domain is a pure Rust module.

2. **Use cases depend on port traits, never on adapters.** A use case never imports from `adapters/`. It calls a `&dyn PortTrait`.

3. **Inbound adapters never contain business logic.** An HTTP handler's only job is: parse input → call use case → serialize output. No `if buyer.rut is None` inside a handler.

4. **Outbound adapters never contain business logic.** A SQLite repository's only job is: translate trait call → SQL → domain entity. No rules about what constitutes a valid DTE inside a repository.

5. **DTOs never reach the use case layer.** The handler converts DTO → domain type before calling the use case. The use case never imports from `adapters/in/dto/`.

6. **All async port trait methods use `#[async_trait]`.** Rust's native async traits (Rust 1.75+) are not yet `dyn`-compatible. Every port trait and its implementations must be annotated with `#[async_trait]`.

7. **All dependency injection is constructor injection.** No global state, no lazy statics for business dependencies. Wire everything in `main.rs`, pass via `AppState`.

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

*Stage 4-1 | Architecture documented: 2026-03-03*
