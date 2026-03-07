# Export / Import — Implementation Plan

References the feature spec in [`export_import.md`](./export_import.md).

## Architecture Decision

The export and import are both handled by dedicated backend endpoints rather than pure frontend JS logic. Reasons:

- No `get_all` method exists on the query services — adding export logic on the frontend would require N+1 requests (fetch all IDs, then fetch each block individually).
- Import conflict resolution requires direct DB access; it belongs in the backend.
- A single `GET /api/export` request returning a zip is simpler for the frontend than assembling one from multiple API calls.

The two new endpoints are:

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/api/export` | Stream a zip archive of all blocks and relations |
| `POST` | `/api/import` | Accept a zip archive and apply import logic |

---

## Step 1 — Storage: add `get_all` to query services and `exists` to repositories

`get_all` is a read-oriented bulk query — it belongs in query services, not repositories. Repositories stay focused on single-entity CRUD (`save`, `get_by_id`, `delete_by_id`). The `exists_by_block_ids` check needed during import is a lightweight point-lookup that fits in the repository layer.

### 1a. DTOs (`storage/src/query_services/`)

Add to `block_query_service/dtos.rs`:

```rust
#[derive(FromRow, Clone, Debug)]
pub struct BlockExportDto {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

Add to `block_link_query_service/dtos.rs`:

```rust
#[derive(FromRow, Clone, Debug)]
pub struct DirectionalLinkExportDto {
    pub id: Uuid,
    pub block_from_id: Uuid,
    pub block_to_id: Uuid,
}

#[derive(FromRow, Clone, Debug)]
pub struct RelatedLinkExportDto {
    pub id: Uuid,
    pub block_a_id: Uuid,
    pub block_b_id: Uuid,
}
```

### 1b. `BlockQueryService` trait (`storage/src/query_services/block_query_service/traits.rs`)

Add:

```rust
async fn get_all<'e, E>(&self, executor: E) -> Result<Vec<BlockExportDto>>
where
    E: Executor<'e, Database = DB>;
```

SQL:
```sql
SELECT id, title, content, created_at, updated_at FROM blocks ORDER BY created_at
```

### 1c. `BlockLinkQueryService` trait (`storage/src/query_services/block_link_query_service/traits.rs`)

Add:

```rust
async fn get_all_directional<'e, E>(&self, executor: E) -> Result<Vec<DirectionalLinkExportDto>>
where
    E: Executor<'e, Database = DB>;

async fn get_all_related<'e, E>(&self, executor: E) -> Result<Vec<RelatedLinkExportDto>>
where
    E: Executor<'e, Database = DB>;
```

SQL (directional):
```sql
SELECT id, block_from_id, block_to_id FROM block_directional_links
```

SQL (related):
```sql
SELECT id, block_a_id, block_b_id FROM block_related_links
```

### 1d. Implement in `storage_sqlite` and `storage_postgres`

Each new trait method gets a concrete SQL impl in both storage crates, following the same pattern as existing methods.

---

## Step 2 — Backend API: Export endpoint

### Directory layout

```
crates/api/src/features/export/
├── mod.rs
├── handler.rs
├── error.rs
└── routes.rs
```

### Handler (`handler.rs`)

```
GET /api/export
```

1. Call `query_services.blocks.get_all(pool)` → `Vec<BlockExportDto>`.
2. Call `query_services.block_links.get_all_directional(pool)` → `Vec<DirectionalLinkExportDto>`.
3. Call `query_services.block_links.get_all_related(pool)` → `Vec<RelatedLinkExportDto>`.
4. Build an in-memory zip using the [`zip`](https://crates.io/crates/zip) crate:
   - `blocks.jsonl`: each block serialised to a JSON line via `serde_json::to_string`.
   - `directional_links.json`: `Vec<[Uuid; 2]>` of `[block_from_id, block_to_id]` pairs.
   - `related_links.json`: `Vec<[Uuid; 2]>` of `[block_a_id, block_b_id]` pairs.
5. Return the zip bytes with headers:
   - `Content-Type: application/zip`
   - `Content-Disposition: attachment; filename="modunote-export.zip"`

### Cargo dependency

Add to `crates/api/Cargo.toml`:

```toml
zip = { version = "2", default-features = false, features = ["deflate"] }
```

### Register route

In `crates/api/src/features/mod.rs`, add `pub mod export;`.

Wire into the main router (wherever `block_links::routes()`, `blocks::routes()` etc. are merged).

---

## Step 3 — Backend API: Import endpoint

### Directory layout

```
crates/api/src/features/import/
├── mod.rs
├── handler.rs
├── error.rs
├── response.rs
└── routes.rs
```

### Handler (`handler.rs`)

```
POST /api/import
Content-Type: multipart/form-data
Field: file  (the .zip binary)
```

1. Extract the `file` multipart field; read bytes into memory.
2. Open as a zip archive (same `zip` crate).
3. **Pre-load existing state** (three queries total, run before processing any imported data):
   - `query_services.blocks.get_all(pool)` → build `HashMap<Uuid, DateTime<Utc>>` of `id → updated_at`.
   - `query_services.block_links.get_all_directional(pool)` → build `HashSet<(Uuid, Uuid)>` of `(from_id, to_id)` pairs.
   - `query_services.block_links.get_all_related(pool)` → build `HashSet<(Uuid, Uuid)>` of `(a_id, b_id)` pairs (already normalised).
4. **Process `blocks.jsonl`**: iterate lines.
   - Deserialise each line into a local `ImportedBlock` struct (`id`, `title`, `content`, `created_at`, `updated_at`).
   - Look up `id` in the pre-loaded map:
     - Missing → call `repos.blocks.save(&block, pool)` preserving original timestamps. Increment `blocks_inserted`.
     - Present, `existing_updated_at >= imported.updated_at` → skip. Increment `blocks_skipped`.
     - Present, `existing_updated_at < imported.updated_at` → overwrite title/content/updated_at, call `repos.blocks.save`. Increment `blocks_updated`.
5. **Process `directional_links.json`**: deserialise as `Vec<[Uuid; 2]>`.
   - For each `[from_id, to_id]`:
     - In the pre-loaded `HashSet` → skip. Increment `dir_links_skipped`.
     - Not present → call `repos.block_directional_links.create(&dto, pool)` with a freshly generated `id`. Increment `dir_links_inserted`. If either block ID is absent from the DB (not in the pre-loaded block map and not just inserted), skip and log a warning.
6. **Process `related_links.json`**: deserialise as `Vec<[Uuid; 2]>`.
   - Normalise each pair (smaller UUID first).
   - Check pre-loaded `HashSet`, then `create` or skip — same pattern as above.
6. Return `200 OK` with `ImportSummary` JSON:
   ```json
   {
     "blocksInserted": 12,
     "blocksUpdated": 3,
     "blocksSkipped": 5,
     "dirLinksInserted": 8,
     "dirLinksSkipped": 2,
     "relatedLinksInserted": 4,
     "relatedLinksSkipped": 1
   }
   ```

### Axum multipart

Add to `crates/api/Cargo.toml` if not already present:

```toml
axum = { version = "...", features = ["multipart"] }
```

### Register route

Same as export — add `pub mod import;` in `features/mod.rs` and merge `import::routes()` into the router.

---

## Step 4 — Frontend: Export button

**File to edit:** the block editor toolbar component.

1. Add an "Export All" button (icon + label).
2. On click:
   ```ts
   const res = await fetch(`${API_URL}/api/export`);
   const blob = await res.blob();
   const url = URL.createObjectURL(blob);
   const a = document.createElement("a");
   a.href = url;
   a.download = `modunote-export-${new Date().toISOString()}.zip`;
   a.click();
   URL.revokeObjectURL(url);
   ```
3. Show a loading spinner on the button while the request is in flight.
4. On error, show an error toast.

No new dependencies needed.

---

## Step 5 — Frontend: Import button

**File to edit:** same toolbar component.

1. Add a hidden `<input type="file" accept=".zip">` and an "Import" button that programmatically triggers it.
2. On file selected:
   ```ts
   const form = new FormData();
   form.append("file", file);
   const res = await fetch(`${API_URL}/api/import`, { method: "POST", body: form });
   const summary = await res.json();
   ```
3. Show a result toast summarising the import (e.g. "12 blocks added, 3 updated, 5 unchanged").
4. After a successful import, refetch/refresh the block graph so newly imported blocks appear.
5. On error, show an error toast with the server message.

---

## Implementation Order

```
Step 1a     Add export DTOs to block_query_service and block_link_query_service
Step 1b–1c  Add get_all / get_all_directional / get_all_related to query service traits
Step 1d     Implement all new trait methods in storage_sqlite + storage_postgres
Step 2      Export endpoint (query_services.blocks.get_all + query_services.block_links.get_all_*)
Step 3      Import endpoint (pre-load 3 queries → in-memory maps/sets → repos.*_links.create)
Step 4      Frontend export button
Step 5      Frontend import button
```

Steps 2 and 3 both depend on Step 1 being done first. Steps 4 and 5 depend on Steps 2 and 3 respectively. Within each step, the order is: trait → SQL impl → handler → route registration.
