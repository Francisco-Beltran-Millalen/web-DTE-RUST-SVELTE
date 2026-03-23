# DTE Auto-Upload — Design Summary (Phase 2)

*Phase 2, Stage 4 | Generated: 2026-03-01*
*Consolidation of all Sketching & Data Modeling artifacts*

---

## 1. Entity Model

### Entities Overview

| Entity | Type | Description |
|--------|------|-------------|
| User | Core | Person who registers to use the service |
| DTE | Core | Tax document generated from a sale and uploaded to SII |
| Sale | Core | Transaction received via webhook from a connected shop |
| SaleItem | Core | Line item detail within a sale |
| Buyer | Core | Person or business that made the purchase (determines boleta vs factura) |
| Shop | Core | E-commerce store on a platform connected by the user |
| SII Credential | Core | User's credentials for authenticating with the SII portal |
| Seller Info | Core | Official business identity that appears on DTEs (1:1 with SII Credential) |
| Sales Channel | Supporting | Platform integration layer (OAuth tokens, webhook config) |
| Calculation Details | Supporting | Audit trail for DTE math (neto, IVA, total) |
| DTE Document | Supporting | Generated file (PDF, XML) from a DTE |
| Notification | Supporting | Messages sent to users about system events |
| User Profile | Supporting | Optional user profile data (phone, bio, social links) |
| User Social Link | Supporting | Flexible social network links on profile |
| Region | Reference | Chilean geographic region |
| Provincia | Reference | Province within a region |
| Ciudad | Reference | City within a province |
| Comuna | Reference | Municipality within a city (used in DTE buyer/seller address) |

**Total: 18 entities** (8 core + 6 supporting + 4 reference)

### Entity Relationships

```
User
 ├── connects → SalesChannel (1:many, V1: 1)
 ├── stores → SII Credential (1:many, V1: 1)
 ├── has → UserProfile (1:1, optional)
 └── receives ← Notification (1:many)

SII Credential
 └── has → Seller Info (1:1)

SalesChannel
 └── manages → Shop (1:many, V1: 1)
               └── linked to → SII Credential (many:1)

Shop
 └── receives → Sale (1:many)

Sale
 ├── has → Buyer (1:1, snapshot per sale)
 ├── contains → SaleItem (1:many)
 └── generates → DTE (1:1)

DTE
 ├── has → Calculation Details (1:1)
 ├── has → DTE Document (1:many)
 └── uses → SII Credential (many:1)

Region → Provincia → Ciudad → Comuna
```

Entity diagram: `docs/assets/diagrams/entity-diagram.md` (Mermaid ER diagram)

### Entity-View Mapping

| View | Entities Used | Purpose |
|------|---------------|---------|
| Layout | User, Notification | Shared sidebar, notification bell |
| Inicio | DTE, Sale, Shop, Buyer, SII Credential | DTE list with all states via tabs |
| Detalle DTE | DTE, Sale, Buyer, Seller Info, Calculation Details, DTE Document | Full SII document view |
| Tiendas | Shop, Sales Channel | Shop connection management |
| Credenciales SII | SII Credential, Seller Info | SII credentials and seller setup |
| Perfil | User, User Profile, User Social Link | Account and profile management |
| Notificaciones | Notification | Notification list |
| Auth views | User | Login, register, password reset |

Full mapping: `docs/view-entity-mapping.md`

---

## 2. Data Model

### Agnostic Model

18 conceptual entities organized into 6 groups: User & Profile, Business & Credentials, Platform & Shops, Sales, DTE & Documents, Notifications.

Key design decisions:
- **Buyer is a snapshot per sale** — stored per sale, not shared across sales
- **DTE has a direct FK to SII Credential** — for query convenience
- **Seller Info is 1:1 with SII Credential** — the official business identity on every DTE
- **Geographic hierarchy** — Region → Provincia → Ciudad → Comuna; only `comuna_id` stored on Buyer and Seller Info

Full document: `docs/data-model-conceptual.md`

### SQLite Physical Model

**19 tables** organized in the same 6 groups.

| Group | Tables |
|-------|--------|
| Geographic Reference | region, provincia, ciudad, comuna |
| User & Profile | user, user_profile, user_social_link, refresh_token |
| Business & Credentials | sii_credential, seller_info |
| Platform & Shops | sales_channel, shop |
| Sales | sale, sale_item, buyer |
| DTE & Documents | dte, calculation_details, dte_document |
| Notifications | notification |

Key implementation decisions:
- Money as INTEGER (CLP has no fractional units)
- Timestamps as TEXT (ISO 8601)
- Enums as TEXT + CHECK constraints
- Encrypted fields (SII password, OAuth tokens) stored as AES-256-GCM ciphertext
- JWT auth: stateless access token (1h) + DB-stored refresh token (7d, `refresh_token` table)
- Soft delete on `user`, `sii_credential`, `shop` — business data retained
- 26 indexes total (17 FK + 9 query performance)

Full document: `docs/data-model-physical.md`

### Database Script

Location: `docs/assets/schema.sql`
- Creates 19 tables with all constraints
- Creates 26 indexes
- Includes seed data covering representative states
- Verified: executes cleanly in SQLite
- Mock data covers 5 DTE states: pendiente, emitido, emision_manual_requerida, fallido, emitido_manualmente
- Note: `emitiendo` is transient (Tokio async task) — not seeded

---

## 3. UI Sketches

### View Inventory

| # | View | File | Category |
|---|------|------|----------|
| 1 | Layout (shared structure) | `layout.html` | Shared |
| 2 | Inicio | `home.html` | Principal |
| 3 | Detalle DTE | `dte-detail.html` | Principal |
| 4 | Tiendas | `tiendas.html` | Configuracion |
| 5 | Credenciales SII | `sii-credentials.html` | Configuracion |
| 6 | Perfil | `profile.html` | Configuracion |
| 7 | Notificaciones | `notifications.html` | Notificaciones |
| 8 | Crear Cuenta | `register.html` | Autenticacion |
| 9 | Iniciar Sesion | `login.html` | Autenticacion |
| 10 | Recuperar Contrasena | `reset-password.html` | Autenticacion |
| 11 | Cambiar Contrasena | `cambiar-contrasena.html` | Autenticacion* |
| 12 | Index (navegador) | `index.html` | Navegacion |

**12 HTML files** (11 views + 1 navigation index). All files present in `docs/assets/views/`.

> *`cambiar-contrasena.html` is listed as Autenticacion in the inventory but is an authenticated view with sidebar. Must be styled as a Configuracion sub-view in Phase 3 (not full-page centered layout).

### Navigation Flow

```
login / register ──────────────────────> home.html
reset-password ─────────────────────────> login.html
home.html ───────────────────────────────> dte-detail.html
dte-detail.html ─────────────────────────> home.html (Volver)
profile.html ────────────────────────────> cambiar-contrasena.html
tiendas.html ────────────────────────────> MercadoLibre OAuth → tiendas.html
notifications.html ──────────────────────> dte-detail.html / tiendas.html
```

Open `docs/assets/views/index.html` in browser to explore all sketches.

---

## 4. API Contract

### Architecture

REST JSON API (Rust + Axum backend, SvelteKit SPA frontend)

| Property | Value |
|----------|-------|
| Base URL | `/api/v1/` |
| Auth | JWT Bearer (access, 1h) + HttpOnly cookie (refresh, 7d) |
| Format | JSON (all endpoints) |

### Endpoints Summary

| Resource Group | Endpoints |
|----------------|-----------|
| Auth | POST /login, /logout, /refresh, /register, /password/reset-request, /password/reset |
| Profile & Account | GET/PUT /profile, PUT /profile/photo, PUT /account/password, PUT /account/email, DELETE /account |
| DTEs | GET /dtes (list+filter), GET /dtes/{id}, POST /dtes/{id}/upload, POST /dtes/{id}/mark-manual, POST /dtes/bulk-upload, GET /dtes/{id}/documents/{docId} |
| Shops | GET /shops, POST /shops/{id}/disconnect, POST /shops/{id}/reconnect, GET /shops/mercadolibre/connect, GET /shops/mercadolibre/callback |
| SII Credentials | GET/POST /sii-credentials, PUT/DELETE /sii-credentials/{id}, POST /sii-credentials/{id}/validate |
| Reference | GET /reference/comunas (autocomplete) |
| Notifications | GET /notifications, POST /notifications/mark-all-read, GET /notifications/unread-count |
| Webhook | POST /webhooks/mercadolibre |

**Total: 33 endpoints** (7 public, 25 authenticated, 1 cookie-authenticated)

### Async Pattern

Two operations use Tokio background tasks with SPA polling:

| Operation | Trigger | Poll endpoint | Terminal states |
|-----------|---------|---------------|-----------------|
| DTE upload | POST /dtes/{id}/upload → 202 | GET /dtes/{id} | `emitido`, `emitido_manualmente`, `fallido` |
| Credential validation | POST /sii-credentials/{id}/validate → 202 | GET /sii-credentials | `validated`, `error` |

### View-Endpoint Mapping Summary

| View | Key Endpoints Called |
|------|---------------------|
| Layout | GET /notifications/unread-count, POST /auth/logout |
| Inicio | GET /dtes (filtered), GET /dtes/{id}, POST /dtes/{id}/upload, POST /dtes/{id}/mark-manual, POST /dtes/bulk-upload |
| Detalle DTE | GET /dtes/{id}, POST /dtes/{id}/upload, POST /dtes/{id}/mark-manual, GET /dtes/{id}/documents/{docId} |
| Tiendas | GET /shops, POST /shops/{id}/disconnect, GET /shops/mercadolibre/connect, POST /shops/{id}/reconnect |
| Credenciales SII | GET/POST/PUT/DELETE /sii-credentials, POST /sii-credentials/{id}/validate, GET /reference/comunas |
| Perfil | GET/PUT /profile, PUT /profile/photo, PUT /account/email, DELETE /account |
| Notificaciones | GET /notifications (filtered), POST /notifications/mark-all-read |
| Auth views | POST /auth/register, /auth/login, /auth/password/reset-request, /auth/password/reset |
| Cambiar Contrasena | PUT /account/password, POST /auth/password/reset-request |

Full API reference: `docs/api-design.md`

---

## 5. Document Templates

The system generates two DTE document types:

| Document | Format | V1 Status |
|----------|--------|-----------|
| DTE XML | XML | Generated server-side, submitted to SII |
| DTE PDF | PDF | Generated server-side, stored in `dte_document` table, available for user download |

> **Email delivery (V2 scope):** Sending PDF to buyer/seller by email is out of scope for V1.
> Per phase-1-consolidation: "Email notifications out of scope for V1."

Reference templates folder: `docs/assets/templates/` (physical reference PDFs to be added)

Full document: `docs/document-templates.md`

---

## 6. Assets Checklist

- [x] `docs/assets/views/` — 12 HTML files (11 views + 1 navigation index)
- [x] `docs/assets/schema.sql` — SQLite script (19 tables, 26 indexes, mock data); verified clean execution
- [x] `docs/assets/diagrams/entity-diagram.md` — Mermaid ER diagram
- [x] `docs/assets/templates/` — Directory exists (reference PDFs to be added in Phase 4)
- [x] `docs/assets/css/styles.css` — Shared styles for HTML sketches

---

## 7. Audit — Issues Fixed During Consolidation

The following inconsistencies were found and corrected during this consolidation:

| # | File | Issue | Fix |
|---|------|-------|-----|
| 1 | `docs/document-templates.md` | Referenced Java PDF library (JasperReports/iText) — old tech stack | Updated to Rust PDF library |
| 2 | `docs/document-templates.md` | Data source table used wrong attribute `nombre` for Buyer razon social | Fixed to `razon_social` |
| 3 | `docs/document-templates.md` | Delivery flow described email-to-buyer/seller as V1 scope; email is V2 per phase-1-consolidation | Clarified V1 vs V2 scope in delivery flow |
| 4 | `docs/assets/schema.sql` | Comment referenced "Spring @Async" — old tech stack | Updated to "Tokio async task" |
| 5 | `docs/assets/templates/` | Directory was missing despite being referenced in document-templates.md | Directory created |
| 6 | `docs/phase-3-design-decisions.md` | Said "No SPA framework — htmx + vanilla JS is sufficient" — contradicts ADR-004 (SvelteKit SPA). Phase 3 HTML views were designed before the 2026-03-01 tech stack change. | Added [REVISED] note: HTMX is prototype-only; production is SvelteKit SPA |
| 7 | `docs/use-cases.md` | Missing use cases that emerged during Phase 2 design: mark-manual, reconnect shop, validate credentials, profile management, notifications, DTE filters by shop/tipo, download document | Added 11 missing use cases to User → Core and User → Standard |
| 8 | `docs/assets/schema.sql` | `emitido_manualmente` DTE state not in seed data; Phase 4 would not see this state without manually triggering the endpoint | Added Sale 5 / SaleItem 5 / Buyer 5 / DTE 5 (emitido_manualmente) / CalculationDetails 5 for Carlos's shop |

---

## 8. Ready for Phase 3

This design package is ready for UI Polish (Phase 3):

- **Views:** All 12 screens are sketched in `docs/assets/views/` — Phase 3 styles these
- **Data:** Entity and data models show exactly what data appears in each view
- **API:** Full REST contracts with JSON examples and view-endpoint mapping
- **Navigation:** Flow documented; sidebar structure clear
- **Key Phase 3 note:** `cambiar-contrasena.html` must use sidebar layout (not auth full-page layout), despite being listed under Autenticacion in the view inventory

Input for Phase 3:
- This document: `consolidation-artifacts/phase-2-consolidation.md`
- `docs/assets/views/` — HTML sketches to style
- `docs/view-entity-mapping.md` — View-to-entity mapping
- `docs/api-design.md` — API contracts
