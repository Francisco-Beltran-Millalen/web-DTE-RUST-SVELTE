# DTE Auto-Upload — Discovery Summary

*Phase 1 Consolidation | Stage 1-6 | Generated: 2026-03-01*
*Input: project-brief.md, knowledge-audit.md, research-findings.md, use-cases.md, tech-stack.md, adrs/*

---

## 1. Project Overview

DTE Auto-Upload is a free web service that helps small Chilean sellers on MercadoLibre comply with SII (Servicio de Impuestos Internos) tax requirements. The service captures sales data, generates the corresponding DTE (Documento Tributario Electrónico) documents, and lets the user upload them to SII with a single action — individually or in batches. The service is free and acts as a lead generator: it collects seller and sales data to enable future paid offerings for users whose businesses grow.

**Core problem:** Small sellers are legally required to submit DTEs for every sale, but the only free method — the SII web portal — is tedious and time-consuming. Paid automation tools exist but are unaffordable for low-volume sellers.

**Target users:** Small-volume Chilean sellers ("minoristas") selling on MercadoLibre who cannot justify paying for automation but must comply with SII requirements.

**Success criteria:**
1. Growing number of active users relying on the service
2. Users save significant time on DTE compliance
3. Brand familiarity with helpful tools (future conversion potential)
4. Database of seller emails and sales data for future paid services

---

## 2. Development Scope

### In Scope (V1 — Mock Data Prototype)

V1 is a fully functional prototype built against mock data. No live external integrations. The core system, UI, and data model are validated before wiring up real services.

- **User authentication**: registration, login, logout, password reset, email update, account deletion
- **SII credential management**: store, update, and remove SII credentials (encrypted at rest); validate format on save
- **MercadoLibre shop connection**: connect and disconnect a shop (simulated OAuth in V1)
- **Sale data ingestion**: receive and store sale data from MercadoLibre (mock data in V1)
- **DTE generation**: generate DTE from sale data (mock in V1)
- **DTE dashboard (home view)**: table of all DTEs with status (Pendiente, Emision manual requerida, Emitiendo, Emitido, Emitido manualmente, Fallido); tabbed views (Pendientes / Emision manual / Fallidos / Emitidos / Todos)
- **Single DTE upload**: user selects one DTE → contextual action button → simulated upload to SII
- **Batch DTE upload**: user selects multiple DTEs → "Upload batch" action → simulated upload
- **Retry failed upload**: retry a DTE in Fallido state
- **DTE history and filters**: filter by status and date range; change display order
- **DTE detail view**: full SII-format view of a single DTE
- **Consent and privacy**: minimal consent checkbox at registration + mock privacy policy

### Out of Scope (V2+)

| Feature | Reason |
|---------|--------|
| Real MercadoLibre OAuth + webhook integration | V2 — validate core system first |
| Real SII upload via Selenium bot | V2 — Java bot exists but integration deferred |
| Shopify integration | V2 — no native RUT support; adds complexity |
| Scheduled / automatic DTE uploads | Not planned (V1 or V2) |
| Reconciliation system (sales vs DTE count) | V2 — complex cross-referencing |
| Email notifications | V2 — status shown in UI table only |
| Admin UI | V2 — admin operations done directly in DB |
| Multiple SII credentials per user | V2 — one business per user in V1 |
| Mobile application | Out of scope |
| Advanced reporting / analytics | Out of scope |
| Paid / premium tier | Out of scope for V1 |

### Explicit Boundaries

- V1 does not connect to real external systems — all integrations are simulated
- No automatic retry of failed uploads — manual retry only
- No email notifications of any kind — status visible in UI table only
- No Admin actor in V1 — admin tasks done directly in the database
- Single MercadoLibre shop per user in V1 — data model supports more, UI does not

---

## 3. Data Landscape

### Available Data (Existing or Researchable)

| Data | Source | Notes |
|------|--------|-------|
| Sale order data | MercadoLibre API (`GET /orders/{id}`) | V2; mocked in V1 |
| Buyer fiscal data (RUT, name, address, tax type) | MercadoLibre API (`GET /orders/{id}/billing_info`) | V2; mocked in V1 |
| OAuth tokens for ML connection | MercadoLibre OAuth flow | V2; simulated in V1 |
| Chilean geographic reference (region/provincia/ciudad/comuna) | Public SII / INE data | Must seed DB |

### Data the System Must Generate

| Data | How |
|------|-----|
| DTE document (XML/PDF) | Generated from sale + buyer + seller data |
| DTE calculation details (neto, IVA, exento, total) | Computed from sale items |
| Upload status and history | Tracked in `dte.estado` and `dte.error_message` |
| Notifications | Generated on upload success/failure/connection error |
| User platform credentials (hashed passwords) | Created at registration |

### Data Requiring Subsystems

| Data | Required Subsystem | V1 Approach |
|------|--------------------|-------------|
| Sale data from MercadoLibre | Webhook receiver + ML API client | Mock seeded in DB |
| SII upload result | SII Selenium bot integration | Simulated async process |
| MercadoLibre OAuth tokens | OAuth 2.0 flow handler | Simulated connection |

---

## 4. Use Cases (Prioritized)

### Actors

| Actor | Description |
|-------|-------------|
| **User** | Small Chilean seller who uses the service to comply with SII requirements |
| **System** | Automation layer: ingests sales (mock), generates DTEs, processes uploads (simulated), tracks status |
| **Admin** | *(V2 — deferred. No UI in V1; operations performed directly in DB.)* |

### Design Priority 1 — Core Business

| # | Use Case |
|---|----------|
| 1 | User can connect their MercadoLibre account |
| 2 | User can disconnect their MercadoLibre account |
| 3 | User can view their connected shops |
| 4 | User can store their SII credentials |
| 5 | User can update their SII credentials |
| 6 | User can remove their SII credential |
| 7 | User can see all DTEs pending to upload |
| 8 | User can upload a single DTE to SII |
| 9 | User can manually trigger a batch upload |
| 10 | User can retry a failed DTE upload |
| 11 | System receives sale notifications from MercadoLibre (mock in V1) |
| 12 | System stores sales data |
| 13 | System generates a DTE from sale data |
| 14 | System encrypts SII credentials |
| 15 | System validates SII credentials when stored |
| 16 | System batches multiple DTEs for upload to SII |
| 17 | System shows upload status in the UI (pending / uploading / uploaded / failed) |

### Design Priority 2 — Supporting

| # | Use Case |
|---|----------|
| 1 | User can view their DTE upload history |
| 2 | User can view details of a specific DTE |
| 3 | User can filter DTEs by status |
| 4 | User can filter DTEs by date range |
| 5 | User can change the display order of DTEs |

### Design Priority 3 — Standard Patterns

| # | Use Case |
|---|----------|
| 1 | User can create an account |
| 2 | User can sign in |
| 3 | User can sign out |
| 4 | User can change their password |
| 5 | User can reset their password via email |
| 6 | User can update their email address |
| 7 | User can delete their account |

---

## 5. Technology Stack

### Summary

| Category | Choice | Version |
|----------|--------|---------|
| Backend language | Rust | 1.78+ stable |
| Backend framework | Axum | 0.7 |
| Async runtime | Tokio | 1.x |
| Prototype DB | SQLite | 3 |
| Production DB | PostgreSQL | 16 |
| DB access + migrations | SQLx + sqlx-cli | 0.8 |
| Authentication | JWT (jsonwebtoken crate) | 9.x |
| Password hashing | argon2 | 0.5 |
| SII encryption | aes-gcm | 0.10 |
| Frontend framework | SvelteKit (SPA mode) | 2.x |
| Frontend language | TypeScript + Svelte 5 | — |
| CSS framework | Tailwind CSS | 3.x |
| Backend testing | Rust built-in + axum-test | — |
| Frontend testing | Vitest + Playwright | — |
| Backend deployment | Google Cloud Run | — |
| Frontend deployment | Cloudflare Pages | — |
| Production DB hosting | Neon (serverless PostgreSQL) | — |

### Key Architectural Decisions

| ADR | Decision | Rationale |
|-----|----------|-----------|
| ADR-001 | Rust + Axum | Developer choice; small binary for cost-efficient Cloud Run deployment |
| ADR-002 | SQLx + SQLite/PostgreSQL | Async, compile-time checked queries; unified interface for both databases |
| ADR-003 | JWT: short-lived access token (1h) + refresh token (7d, DB-stored, HttpOnly cookie) | Allows token revocation; SII credential access requires revocable auth; API is multi-client (future mobile/desktop) |
| ADR-004 | SvelteKit SPA mode | File-based routing fits ~11 views across two layout groups; Svelte ecosystem default |
| ADR-005 | Cloud Run + Cloudflare Pages + Neon | Scale-to-zero on all layers; near-zero cost for prototype traffic |

### API Contract

- Pure **REST JSON API** — all responses are JSON, no server-rendered HTML
- **JWT Bearer token** in `Authorization` header for all authenticated endpoints
- Refresh token delivered as **HttpOnly cookie** — not accessible to JavaScript
- API designed to be consumed by any client (SPA, mobile, desktop)
- Prefix: `/api/v1/`

---

## 6. Key Decisions Made

| Decision | Rationale |
|----------|-----------|
| V1 is built entirely against mock data | Validate core system, UI, and data model before live integrations |
| Real ML + SII integrations deferred to V2 | Focus on prototype correctness first |
| Upload is user-triggered (manual) in V1 | Simpler; automatic/scheduled deferred to V2+ |
| Single SII credential per user in V1 | Keep V1 simple; data model supports multi-credential for V2 |
| Admin actor deferred to V2 | Admin tasks done directly in DB for prototype |
| Shopify deferred to V2 | No native RUT support; adds complexity without validating core flow |
| Email notifications out of scope for V1 | Status shown in UI table only |
| Reconciliation deferred to V2 | Complex cross-referencing of SII records + sales channel data |
| SII credentials encrypted with AES-256-GCM | OWASP recommended; key loaded from environment variable |
| Minimal consent checkbox + mock privacy policy | Ley 21.719 enforcement starts Dec 2026; outside V1 window |
| Proceed with SII credential storage | No legal prohibition found; explicit consent + encryption is sufficient |
| JWT stateless access token + DB-stored refresh token | Revocable auth required for SII credential access; multi-client API |
| SvelteKit SPA mode (SSR disabled) | Separated frontend consumes REST API; no server rendering needed |

---

## 7. Known Risks & Uncertainties

| Risk | Level | Mitigation |
|------|-------|------------|
| SII Selenium bot blocked by SII portal changes | Low | Accepted risk for V1; monitor; Java bot already working |
| MercadoLibre API rate limits | Low | Implement backoff in V2; not relevant for V1 mock |
| Token refresh edge cases (ML OAuth) | Low | Robust token storage and refresh logic in V2 |
| Exact DTE XML format fields | Low for V1 | Clarify during V2 implementation; mock data only in V1 |
| SII credential storage — no specific regulation found | Medium | Legal opinion before V2 production launch |
| Rust learning curve for developer (beginner level) | Medium | Understood ownership; lifetimes encountered incrementally; Axum is beginner-accessible |
| Neon serverless PostgreSQL connection limits | Low | Direct connections acceptable for V1; add connection pooling in V2 |
| Cloud Run cold starts for infrequent prototype use | Low | Rust binary minimizes image size and startup time |

---

## 8. Open Questions (Deferred)

| Question | When to Resolve |
|----------|----------------|
| Exact DTE XML format and required fields | During V2 implementation |
| Encryption key rotation strategy for SII credentials | Before V2 production launch (migrate to GCP Secret Manager / AWS Secrets Manager) |
| Full Ley 21.719 compliance review | Before V2 production launch |
| Legal opinion on SII credential storage | Before V2 production launch |
| MercadoLibre OAuth edge cases (token refresh, expiry) | During V2 implementation |
| Shopify API + RUT workaround research | V2 planning |
| Reconciliation mechanism design | V2 planning |

---

*Phase 1 complete. Proceed to Phase 2: Sketching & Data Modeling (Stage 2-1: Entity & UI Sketching).*
*Primary input for Phase 2: this document.*
