-- ============================================================
-- DTE Auto-Upload - SQLite Schema with Mock Data
-- Generated: 2026-02-21 | Updated: 2026-03-01
-- Stage: 2-2 - Data Modeling
-- Tables: 19
-- Auth: JWT (Axum/jsonwebtoken) — short-lived access token (stateless) + refresh_token table (7d, HttpOnly cookie)
-- Async: Tokio background tasks — no task_queue table
-- ============================================================

-- NOTE: SQLite disables foreign key enforcement by default.
-- Application code must run this on every connection:
PRAGMA foreign_keys = ON;

-- ============================================================
-- GEOGRAPHIC REFERENCE TABLES
-- ============================================================

CREATE TABLE region (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE provincia (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    region_id INTEGER NOT NULL REFERENCES region(id) ON DELETE RESTRICT,
    name TEXT NOT NULL
);

CREATE TABLE ciudad (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provincia_id INTEGER NOT NULL REFERENCES provincia(id) ON DELETE RESTRICT,
    name TEXT NOT NULL
);

CREATE TABLE comuna (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ciudad_id INTEGER NOT NULL REFERENCES ciudad(id) ON DELETE RESTRICT,
    name TEXT NOT NULL
);

-- ============================================================
-- USER & PROFILE
-- ============================================================

CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    deleted_at TEXT
);

CREATE TABLE user_profile (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE REFERENCES user(id) ON DELETE CASCADE,
    phone TEXT,
    about_me TEXT,
    profile_photo_url TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE user_social_link (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_profile_id INTEGER NOT NULL REFERENCES user_profile(id) ON DELETE CASCADE,
    platform TEXT NOT NULL,
    url TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(user_profile_id, platform)
);

CREATE TABLE refresh_token (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TEXT NOT NULL,
    revoked_at TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ============================================================
-- BUSINESS & CREDENTIALS
-- ============================================================

CREATE TABLE sii_credential (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE RESTRICT,
    rut TEXT NOT NULL,
    password_encrypted TEXT NOT NULL,
    validation_status TEXT NOT NULL DEFAULT 'not_validated'
        CHECK(validation_status IN ('validated', 'not_validated', 'error')),
    validated_at TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    deleted_at TEXT
);

CREATE TABLE seller_info (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sii_credential_id INTEGER NOT NULL UNIQUE REFERENCES sii_credential(id) ON DELETE CASCADE,
    razon_social TEXT NOT NULL,
    giro TEXT NOT NULL,
    direccion TEXT NOT NULL,
    comuna_id INTEGER NOT NULL REFERENCES comuna(id) ON DELETE RESTRICT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ============================================================
-- PLATFORM & SHOPS
-- ============================================================

CREATE TABLE sales_channel (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE RESTRICT,
    platform TEXT NOT NULL CHECK(platform IN ('mercadolibre', 'shopify')),
    platform_user_id TEXT NOT NULL,
    access_token_encrypted TEXT,
    refresh_token_encrypted TEXT,
    token_expires_at TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'error')),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE shop (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sales_channel_id INTEGER NOT NULL REFERENCES sales_channel(id) ON DELETE RESTRICT,
    sii_credential_id INTEGER NOT NULL REFERENCES sii_credential(id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    platform_shop_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'activa'
        CHECK(status IN ('activa', 'desconectada', 'error_conexion')),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    deleted_at TEXT,
    UNIQUE(sales_channel_id, platform_shop_id)
);

-- ============================================================
-- SALES
-- ============================================================

CREATE TABLE sale (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    shop_id INTEGER NOT NULL REFERENCES shop(id) ON DELETE RESTRICT,
    platform_order_id TEXT NOT NULL,
    order_date TEXT NOT NULL,
    total_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'CLP',
    platform_status TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(shop_id, platform_order_id)
);

CREATE TABLE sale_item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_id INTEGER NOT NULL REFERENCES sale(id) ON DELETE CASCADE,
    sku TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit TEXT NOT NULL DEFAULT 'UN',
    unit_price INTEGER NOT NULL,
    discount_percent REAL NOT NULL DEFAULT 0,
    indicator TEXT NOT NULL DEFAULT 'afecto'
        CHECK(indicator IN ('afecto', 'exento')),
    total INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE buyer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_id INTEGER NOT NULL UNIQUE REFERENCES sale(id) ON DELETE CASCADE,
    rut TEXT,
    razon_social TEXT,
    giro TEXT,
    direccion TEXT,
    comuna_id INTEGER REFERENCES comuna(id) ON DELETE SET NULL,
    email TEXT,
    buyer_type TEXT NOT NULL CHECK(buyer_type IN ('persona_natural', 'empresa')),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ============================================================
-- DTE & DOCUMENTS
-- ============================================================

CREATE TABLE dte (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_id INTEGER NOT NULL UNIQUE REFERENCES sale(id) ON DELETE RESTRICT,
    sii_credential_id INTEGER NOT NULL REFERENCES sii_credential(id) ON DELETE RESTRICT,
    tipo TEXT NOT NULL CHECK(tipo IN ('boleta_electronica', 'factura_electronica')),
    folio TEXT,
    estado TEXT NOT NULL DEFAULT 'pendiente'
        CHECK(estado IN ('pendiente', 'emision_manual_requerida', 'emitiendo',
                         'emitido', 'emitido_manualmente', 'fallido')),
    fecha_emision TEXT,
    observations TEXT,
    payment_method TEXT,
    payment_amount INTEGER,
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE calculation_details (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dte_id INTEGER NOT NULL UNIQUE REFERENCES dte(id) ON DELETE CASCADE,
    neto INTEGER NOT NULL,
    iva INTEGER NOT NULL,
    exento INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE dte_document (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dte_id INTEGER NOT NULL REFERENCES dte(id) ON DELETE CASCADE,
    document_type TEXT NOT NULL CHECK(document_type IN ('pdf', 'xml')),
    file_path TEXT NOT NULL,
    file_size INTEGER,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ============================================================
-- NOTIFICATIONS
-- ============================================================

CREATE TABLE notification (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    dte_id INTEGER REFERENCES dte(id) ON DELETE SET NULL,
    shop_id INTEGER REFERENCES shop(id) ON DELETE SET NULL,
    notification_type TEXT NOT NULL
        CHECK(notification_type IN ('upload_success', 'upload_failed',
                                     'manual_upload_required', 'connection_error')),
    message TEXT NOT NULL,
    is_read INTEGER NOT NULL DEFAULT 0,
    action_url TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ============================================================
-- INDEXES - Foreign Keys (17)
-- ============================================================

CREATE INDEX idx_provincia_region_id ON provincia(region_id);
CREATE INDEX idx_ciudad_provincia_id ON ciudad(provincia_id);
CREATE INDEX idx_comuna_ciudad_id ON comuna(ciudad_id);

CREATE INDEX idx_refresh_token_user_id ON refresh_token(user_id);

CREATE INDEX idx_sii_credential_user_id ON sii_credential(user_id);
CREATE INDEX idx_seller_info_comuna_id ON seller_info(comuna_id);

CREATE INDEX idx_sales_channel_user_id ON sales_channel(user_id);
CREATE INDEX idx_shop_sales_channel_id ON shop(sales_channel_id);
CREATE INDEX idx_shop_sii_credential_id ON shop(sii_credential_id);

CREATE INDEX idx_sale_shop_id ON sale(shop_id);
CREATE INDEX idx_sale_item_sale_id ON sale_item(sale_id);
CREATE INDEX idx_buyer_comuna_id ON buyer(comuna_id);

CREATE INDEX idx_dte_sii_credential_id ON dte(sii_credential_id);
CREATE INDEX idx_dte_document_dte_id ON dte_document(dte_id);

CREATE INDEX idx_notification_user_id ON notification(user_id);
CREATE INDEX idx_notification_dte_id ON notification(dte_id);
CREATE INDEX idx_notification_shop_id ON notification(shop_id);

-- ============================================================
-- INDEXES - Query Performance (9)
-- ============================================================

CREATE INDEX idx_dte_estado ON dte(estado);
CREATE INDEX idx_dte_tipo ON dte(tipo);
CREATE INDEX idx_shop_status ON shop(status);
CREATE INDEX idx_notification_is_read ON notification(is_read);
CREATE INDEX idx_sale_platform_order_id ON sale(platform_order_id);
CREATE INDEX idx_user_deleted_at ON user(deleted_at);
CREATE INDEX idx_sii_credential_deleted_at ON sii_credential(deleted_at);
CREATE INDEX idx_shop_deleted_at ON shop(deleted_at);
CREATE INDEX idx_refresh_token_expires_at ON refresh_token(expires_at);
