# DTE Auto-Upload — UI Style Guide

> **Phase 4 Reference Document**
> This style guide is the authoritative visual reference for all SvelteKit component development in Phase 4. The HTML views in `docs/assets/views/` are design references only — do not copy HTMX attributes or inline scripts from them.

---

## 1. Design Philosophy

**Warm, professional, minimal.** The UI targets small business owners managing tax documents (DTEs). The visual language is calm and trustworthy — not clinical, not loud.

Key principles:
- **Clarity first** — every action and status must be immediately legible
- **Error transparency** — every failure must display `FALLIDO` + specific reason (no silent failures, no generic messages)
- **Desktop-focused** — V1 targets 1024px minimum width; mobile deferred to V2
- **Warm neutrals over grays** — the palette uses off-whites and warm browns, not cool grays
- **No decorative noise** — icons, borders, and badges serve function; no ornamental elements

---

## 2. Framework & Tools

### Reference views (Phase 3 design prototype)
| Tool | Version | CDN |
|------|---------|-----|
| Tailwind CSS | CDN (Play) | `https://cdn.tailwindcss.com` |
| Inter font | — | `https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap` |
| htmx | 2.0.4 | `https://unpkg.com/htmx.org@2.0.4` (visual refs only — **do not use in Phase 4**) |

### Phase 4 implementation

> **[REVISED 2026-03-22] — Tailwind CSS dropped. Pure CSS.**
> Tailwind CSS is dying (framework and ecosystem in decline). All Phase 4 styling uses plain CSS.
> CSS architecture (custom property naming, file structure, scoped vs global) is defined in the Stage 4-1 revisit session.

| Tool | Notes |
|------|-------|
| SvelteKit | SPA routes (`+page.svelte`) |
| ~~Tailwind CSS 3.x~~ **Plain CSS** | CSS custom properties in `app.css` + scoped `<style>` blocks per component |
| Inter font | Self-hosted or Google Fonts via SvelteKit layout |
| No htmx | All interactivity is Svelte state + event handlers |
| API data fetching | `fetch` with custom wrapper (see `docs/tech-stack.md`) |

**Design tokens** (define as CSS custom properties in `app.css` — exact naming conventions set in Stage 4-1 revisit):

| Token | Value |
|-------|-------|
| Primary 50 | `#f0fdf4` |
| Primary 100 | `#dcfce7` |
| Primary 500 | `#22c55e` |
| Primary 600 | `#16a34a` |
| Primary 700 | `#15803d` |
| Primary 800 | `#166534` |
| Primary 900 | `#14532d` |
| warm-bg | `#ece9e4` |
| warm-surface | `#f9f7f4` |
| warm-border | `#cfc9c2` |
| warm-border-light | `#ddd9d3` |
| ink | `#3d3530` |
| ink-secondary | `#7a6f66` |
| ink-muted | `#9c9187` |
| ink-faint | `#b5aba2` |

---

## 3. Colors

### Primary Palette (Green)
| Name | Hex | Tailwind Class | Usage |
|------|-----|----------------|-------|
| Primary 50 | `#f0fdf4` | `bg-primary-50` / `text-primary-50` | Active nav item background, hover tints |
| Primary 100 | `#dcfce7` | `bg-primary-100` | Subtle green tints |
| Primary 500 | `#22c55e` | `text-primary-500` | Focus ring color (via `focus:ring-primary-500/30`) |
| Primary 600 | `#16a34a` | `bg-primary-600` | Primary buttons, logo background, active tab border |
| Primary 700 | `#15803d` | `text-primary-700` | Active nav text, links, hover CTAs |
| Primary 800 | `#166534` | — | Deep green accents |

### Warm Palette (Backgrounds & Borders)
| Name | Hex | CSS Variable | Usage |
|------|-----|-------------|-------|
| warm-bg | `#ece9e4` | `--color-warm-bg` | Page background, filter pill bg, inner panel bg |
| warm-surface | `#f9f7f4` | `--color-warm-surface` | Cards, sidebar, modal background |
| warm-border | `#cfc9c2` | `--color-warm-border` | Card borders (2px), sidebar right border, separators |
| warm-border-light | `#ddd9d3` | `--color-warm-border-light` | Row dividers, section dividers |

### Ink (Text Hierarchy)
| Name | Hex | CSS Variable | Usage |
|------|-----|-------------|-------|
| ink | `#3d3530` | `--color-ink` | Headings, primary body text, bold values |
| ink-secondary | `#7a6f66` | `--color-ink-secondary` | Labels, secondary info, button text |
| ink-muted | `#9c9187` | `--color-ink-muted` | Table headers, helper text, nav inactive icons |
| ink-faint | `#b5aba2` | `--color-ink-faint` | Placeholders, disabled text, date inputs, logout icon |

### Status Colors (DTE States)
| Status | Background | Text | Usage |
|--------|-----------|------|-------|
| Pendiente | `bg-amber-100` | `text-amber-800` | DTE waiting to be processed |
| Emision manual | `bg-orange-100` | `text-orange-800` | Manual emission required |
| Emitiendo | `bg-blue-100` | `text-blue-800` | Emission in progress |
| Emitido | `bg-green-100` | `text-green-800` | Successfully emitted |
| Emitido manual | `bg-emerald-100` | `text-emerald-800` | Emitted manually |
| **Fallido** | `bg-red-100` | `text-red-800` | **Failed — always show reason (see §8 Error Policy)** |

### Sub-Status Colors (PDF)
| Sub-Status | Background | Text |
|------------|-----------|------|
| PDF enviado | `bg-blue-50` | `text-blue-600` |
| PDF pendiente | `bg-amber-50` | `text-amber-700` |

---

## 4. Typography

### Font Stack
```
Primary: 'Inter', system-ui, sans-serif
Monospace: ui-monospace, 'Cascadia Code', 'Source Code Pro', monospace
```

### Scale
| Element | Size | Tailwind | Weight | Usage |
|---------|------|----------|--------|-------|
| Brand name | 20px | `text-xl font-bold` | 700 | Logo text in auth views |
| Page title | 18px | `text-lg font-bold` | 700 | Main view headings |
| Auth heading | 16px | `text-base font-bold` | 700 | Auth card titles |
| Body text | 14px | `text-sm` | 400 | Default content, descriptions |
| Card title | 14px | `text-sm font-semibold` | 600 | Card headings, nav labels |
| Section heading | 11px | `text-xs uppercase tracking-wider font-semibold` | 600 | Section labels inside cards |
| Filter/badge | 11px | `text-xs font-medium` | 500 | Status badges, filter pills, tab counts |
| Sub-badge | 10px | `text-[10px] font-medium` | 500 | PDF sub-status badges |

**Monospace** (`font-mono`): folios, RUTs, document IDs, monetary amounts in SII preview.

---

## 5. Spacing

Base unit: 4px (Tailwind `space-1`)

| Name | Value | CSS Variable | Tailwind | Usage |
|------|-------|-------------|----------|-------|
| space-1 | 4px | `--space-1` | `p-1` | Tight gaps |
| space-1.5 | 6px | — | `p-1.5` | Label-to-input gap, icon-text gaps |
| space-2 | 8px | `--space-2` | `p-2` | Button groups, small padding |
| space-3 | 12px | `--space-3` | `p-3` | Table cell vertical, card section gaps |
| space-4 | 16px | `--space-4` | `p-4` | Card vertical padding, inner panel padding |
| space-5 | 20px | `--space-5` | `p-5` | Card horizontal padding |
| space-6 | 24px | `--space-6` | `p-6` | Page padding, auth card padding |

**Common patterns:**
- Card padding: `px-5 py-4`
- Page padding: `p-6`
- Table cell: `px-5 py-3`
- Form field gap: `space-y-4`
- Section gap: `mb-6`

---

## 6. Components

### Buttons

| Variant | Classes | Usage |
|---------|---------|-------|
| Primary | `bg-primary-600 hover:bg-primary-700 text-white font-medium py-2.5 px-4 rounded-lg text-sm` | Main CTA actions |
| Secondary | `border border-warm-border text-ink-secondary hover:text-ink hover:bg-warm-bg px-4 py-2 rounded-lg text-sm` | Secondary actions, cancel |
| Dark (download) | `bg-ink/80 hover:bg-ink text-warm-surface font-medium px-4 py-2 rounded-lg text-sm` | Download PDF, dark CTA |
| Destructive | `bg-red-600 hover:bg-red-700 text-white font-medium px-4 py-2 rounded-lg text-sm` | Delete, dangerous actions |
| Pill (bulk) | `text-xs bg-primary-600 hover:bg-primary-700 text-white px-3 py-1.5 rounded-full font-medium` | Bulk action bar |
| Full-width (auth) | `w-full bg-primary-600 hover:bg-primary-700 text-white font-medium py-2.5 px-4 rounded-lg text-sm` | Auth form submit |
| Disabled | `bg-warm-bg text-ink-faint font-medium px-4 py-2 rounded-lg cursor-not-allowed` | Inactive/loading state |

### Forms

| Element | Classes |
|---------|---------|
| Text input | `w-full bg-warm-bg/60 border border-warm-border rounded-lg px-4 py-2.5 text-sm text-ink placeholder:text-ink-faint focus:outline-none focus:ring-2 focus:ring-primary-500/30 focus:border-primary-500` |
| Label | `block text-sm font-medium text-ink-secondary mb-1.5` |
| Select pill filter | `text-xs text-ink-secondary bg-warm-bg/60 hover:bg-warm-bg rounded-full pl-7 pr-6 py-1.5 border-0 appearance-none` |
| Date input | `bg-warm-bg/60 text-ink-secondary rounded-full px-2.5 py-1.5 border-0 text-xs` |

**Inline error state** (auth views): `bg-red-50 border border-red-200 rounded-lg px-4 py-2.5` with red icon + `text-red-800` message — shown above submit button.

### Tables

| Element | Classes |
|---------|---------|
| Container | `bg-warm-surface border-2 border-warm-border rounded-xl overflow-hidden` |
| Header row | `sticky top-0 bg-warm-bg/80 z-10` |
| Header cell | `px-5 py-3 font-semibold text-ink-muted` |
| Body | `divide-y divide-warm-border-light` |
| Body row | `hover:bg-warm-bg/40 cursor-pointer transition-colors` |
| Selected row | `bg-primary-50/50` |
| Checkbox column | `w-10` |
| Folio/ID cell | `font-mono text-xs` |

### Badges / Status Indicators

| Variant | Classes |
|---------|---------|
| Status badge | `inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium [+ state-colors]` |
| Sub-badge (PDF) | `inline-flex items-center px-1.5 py-0.5 rounded-full text-[10px] font-medium [+ state-colors]` |
| Tab count (alert) | `ml-0.5 text-[11px] font-bold text-red-800/70` |
| Tab count (neutral) | `ml-0.5 text-[11px] text-ink-faint` |

### Cards / Panels

| Variant | Classes |
|---------|---------|
| Standard card | `bg-warm-surface border-2 border-warm-border rounded-xl overflow-hidden` |
| Detail panel (side) | `w-[400px] bg-warm-surface border-2 border-warm-border rounded-xl ml-4 flex-shrink-0` |
| Inner panel (amounts) | `bg-warm-bg/60 rounded-lg p-4` |
| Error state card | Replace `border-warm-border` with `border-red-300` |
| Section heading | `text-xs font-semibold text-ink-muted uppercase tracking-wider` |
| Danger zone card | `border-red-300` border + `text-red-600` section heading + `bg-red-600` destructive button |

### Navigation (Icon Sidebar)

Fixed 64px sidebar (`w-16`). Icon-only — no text labels; use `title` attributes for tooltips.

```
Structure (top to bottom):
1. Logo  — w-10 h-10 rounded-xl bg-primary-600 (links to home)
2. [mb-4 gap after logo]
3. Inicio icon
4. [separator: w-8 h-px bg-warm-border my-1]
5. Tiendas icon
6. Credenciales SII icon
7. [flex-1 spacer]
8. Notificaciones icon (+ red dot when unread: absolute w-2 h-2 bg-red-500 rounded-full top-0 right-0)
9. Perfil icon
10. Salir icon
```

**Active item:** `bg-primary-50 text-primary-700`
**Inactive item:** `text-ink-muted hover:bg-warm-bg hover:text-ink-secondary`
**Logout/tertiary:** `text-ink-faint hover:bg-warm-bg hover:text-ink-muted`

### Tabs

| State | Classes |
|-------|---------|
| Active | `px-3.5 py-2 rounded-t-lg text-sm font-semibold border-b-2 border-primary-600 text-primary-700 bg-warm-bg/50` |
| Inactive | `px-3.5 py-2 rounded-t-lg text-sm font-medium text-ink-muted hover:text-ink-secondary hover:bg-warm-bg/30 border-b-2 border-transparent` |

### Modals

```
Overlay: fixed inset-0 z-50 flex items-center justify-center
  Backdrop: absolute inset-0 bg-ink/30
  Card: relative bg-warm-surface border-2 border-warm-border rounded-xl p-6 w-full max-w-sm shadow-lg
```

### Alerts / Error Messages

**Rule: Every error must show `FALLIDO` + the specific reason. No silent failures. No generic messages.**

```
Error banner: bg-red-50 border border-red-200 rounded-lg px-4 py-2.5
  Icon: text-red-500 (X or alert icon)
  Title: "FALLIDO" — text-red-800 font-semibold
  Reason: text-red-700 text-sm — specific error description
```

This pattern applies to: DTE emission failures, SII API errors, shop connection failures, credential validation failures, and any other operation result.

### Accordion / Option Cards

Used in `cambiar-contrasena.html`:
- First option open by default; second collapsed
- Chevron rotates via `-rotate-90` when collapsed
- Toggle via JS class manipulation on click

### Notification List Item

| State | Background | Indicator |
|-------|-----------|-----------|
| Unread | `bg-primary-50/20` | `bg-primary-600` filled dot |
| Read | default | empty dot spacer |

Structure: type icon + type badge + timestamp + message + optional action link.

### Profile Avatar

`w-16 h-16 rounded-xl bg-warm-bg` with camera button overlay: `absolute -bottom-1 -right-1 w-6 h-6`.

### Navigation Card (link-to-subview)

`flex items-center gap-3 px-5 py-4 hover:bg-warm-bg/40` with icon, title, description, and `chevron-right` — used for cambiar-contraseña link from profile.

### Filter Pills

| State | Classes |
|-------|---------|
| Active | `bg-ink/80 text-warm-surface` |
| Inactive | `text-ink-secondary bg-warm-bg/60` |

---

## 7. Layouts

### App Layout (Sidebar Views)

```html
<body class="bg-warm-bg min-h-screen flex">
  <aside class="w-16 ... fixed top-0 left-0 h-screen"> <!-- sidebar --> </aside>
  <div class="flex-1 flex flex-col min-h-screen ml-16">
    <main class="p-6 [max-width] mx-auto w-full"> <!-- content --> </main>
  </div>
</body>
```

**Content max-width by view type:**
| Type | Max Width | Views |
|------|-----------|-------|
| Data-heavy | Full / `max-w-5xl` | home, dte-detail |
| List / Settings | `max-w-4xl` | tiendas, notifications |
| Simple form | `max-w-2xl` | sii-credentials, profile, cambiar-contrasena |

### Auth Layout (Standalone Views)

```html
<body class="bg-warm-bg min-h-screen flex flex-col items-center justify-center px-4">
  <!-- Logo block -->
  <div class="flex flex-col items-center mb-8"> ... </div>
  <!-- Card -->
  <div class="w-full max-w-sm bg-warm-surface border-2 border-warm-border rounded-xl p-6"> ... </div>
</body>
```

No sidebar, no htmx. Multi-step flows (reset-password) use JS-driven step switching with `hidden` class toggling.

---

## 8. Error Policy

**Every failed operation must display `FALLIDO` + the specific reason.**

This is a hard requirement, not a guideline. Examples:
- DTE emission failure → show "FALLIDO: [reason from SII API]"
- Shop connection failure → show "FALLIDO: [connection error description]"
- Credential validation failure → show "FALLIDO: [validation error]"

No operation may fail silently. No generic "Something went wrong" messages. The reason must come from the actual error response.

**Visual treatment:** See §6 Alerts / Error Messages for the error banner pattern. The `Fallido` status badge (§3 Status Colors) applies to DTE rows in the table.

---

## 9. Accessibility

- **Color contrast:** Primary text (`ink` #3d3530) on warm-surface (#f9f7f4) → ~9:1 ratio ✅; ink-muted on warm-surface → ~3.5:1 (meets AA for large/UI text)
- **Focus indicators:** `focus:ring-2 focus:ring-primary-500/30 focus:border-primary-500` on all inputs; `:focus-visible` outline via `styles.css`
- **Keyboard navigation:** Sidebar links, tab panels, and modals must be keyboard accessible in Phase 4 implementation
- **Screen readers:** Icon-only sidebar items use `title` attributes (visual tooltip); Phase 4 should add `aria-label` to icon buttons
- **Checkbox:** `accent-color: var(--color-primary-600)` applied globally via `styles.css`
- **Destructive confirmation:** Acknowledgment checkbox (`accent-red-600`) paired with a warning paragraph before the destructive button is enabled

---

## 10. View Inventory

| View | File | Layout | Status |
|------|------|--------|--------|
| Inicio (DTE list + detail) | `home.html` | App (full width) | ✅ Complete — reference implementation |
| Detalle DTE | `dte-detail.html` | App (full width) | ✅ Complete |
| Tiendas | `tiendas.html` | App (max-w-4xl) | ✅ Complete |
| Credenciales SII | `sii-credentials.html` | App (max-w-2xl) | ✅ Complete |
| Perfil | `profile.html` | App (max-w-2xl) | ✅ Complete |
| Cambiar Contraseña | `cambiar-contrasena.html` | App (max-w-2xl) | ✅ Complete — sub-page of Perfil |
| Notificaciones | `notifications.html` | App (max-w-4xl) | ✅ Complete |
| Iniciar Sesión | `login.html` | Auth | ✅ Complete — error state visible in prototype |
| Crear Cuenta | `register.html` | Auth | ✅ Complete |
| Recuperar Contraseña | `reset-password.html` | Auth | ✅ Complete — 4-step flow |
| Index (nav hub) | `index.html` | Standalone | ✅ Complete |
| Layout sketch | `layout.html` | — | ❌ Excluded — superseded by actual views |

---

## 11. Decision Log

### Framework Selection
**Decision:** Tailwind CSS (CDN) + Inter font via Google Fonts for Phase 3 reference views.
**Rationale:** Zero build step for rapid prototyping; utility-first for tight design control.
**Phase 4:** Tailwind 3.x via SvelteKit build pipeline (not CDN).

### No SPA Framework → Revised to SvelteKit
**Original (2026-02-21):** "No SPA framework — htmx + vanilla JS is sufficient."
**Revision (2026-03-01):** HTMX is not used at any level. All interactivity is SvelteKit/Svelte/TypeScript.
**Rationale:** Team skill set and long-term maintainability favor SvelteKit over htmx for the V1 implementation.
**Rule:** HTML files in `docs/assets/views/` are visual design references only. Do not copy HTMX attributes or inline `<script>` blocks into Svelte components.

### Design Language: Warm Neutrals
**Decision:** Warm off-white/brown palette (`warm-*`, `ink-*`) instead of standard cool grays.
**Rationale:** Differentiates from generic SaaS; feels professional and approachable for small business owners; adopted from a prior project with the same design system (see `imported-artifacts/ui-style-guide-imported.md`).

### Color Palette — Primary: Green
**Decision:** Green (`primary-*`) as primary action color.
**Rationale:** Communicates success/approval, aligns with DTE emission completion; trustworthy for financial tooling.

### Navigation: Icon-Only Sidebar
**Decision:** 64px fixed sidebar with icons only (no text labels), `title` tooltips for hover identification.
**Rationale:** Maximizes horizontal space for data-heavy tables; icons are self-explanatory for the small set of nav items.

### Responsive: Desktop-Only for V1
**Decision:** Minimum target width 1024px. Mobile/responsive layout deferred to V2.
**Rationale:** Target users (small sellers managing DTEs) primarily on desktop; data-heavy tables are difficult on mobile; V1 is a prototype. V2 will need a mobile nav pattern and horizontal-scroll or card layout for tables.

### View Scoping: Documentos Emitidos Merged into Home
**Decision:** `documentos-emitidos.html` excluded; content merged into `home.html` as tabbed views (Pendientes / Emitidos).
**Rationale:** Reduces navigation hops; tabs within the home view are more efficient than a separate route.

### Auth Views: Standalone Layout
**Decision:** Auth views (login, register, reset-password) use a centered card layout without sidebar.
**Rationale:** Standard auth UX pattern; sidebar would add noise during unauthenticated state.

### Reset Password: Multi-Step Flow
**Decision:** 4-step flow (email → code → new password → confirmation) in a single view, JS-driven step switching.
**Rationale:** Keeps user in context; avoids multiple page loads during a sensitive flow.

### Danger Zone / Destructive Actions
**Decision:** Danger zone uses red border card, acknowledgment checkbox, and `bg-red-600` button — checkbox must be checked to enable button.
**Rationale:** Prevents accidental destructive actions; clear visual warning before account deletion.

---

## 12. Edge Cases

### Third-Party Communication Failures
**Context:** The system depends on external services (SII DTE API, e-commerce platforms).
**Classification:** Needs UI + Backend concern.

**UI rule (hard requirement):** Every failed operation must display `FALLIDO` + specific reason. See §8 Error Policy and §6 Alerts / Error Messages.

**Backend concern (Phase 4):** Design a reconciliation/coordination mechanism to detect missed sales — cases where the external system processed a sale but our system did not receive confirmation (or vice versa). This is a backend architecture concern beyond UI scope; flagged for Phase 4 design.

### Empty States (New Users / No Data)
**Classification:** Deferred to V2.
**Rationale:** V1 always has mock data; empty state UI is not needed until real users onboard. When V2 addresses this, the pattern should be: "No [items] yet" message + call-to-action button appropriate to the view.
