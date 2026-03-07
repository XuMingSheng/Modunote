# Export / Import

Modunote supports exporting all blocks and their relations as a `.zip` archive and re-importing that archive into any Modunote instance.

## UI Entry Points

| Button | Location | Action |
|--------|----------|--------|
| **Export All** | Block editor toolbar | Downloads `modunote-export-<timestamp>.zip` |
| **Import** | Block editor toolbar | Opens a file picker; accepts `.zip` files produced by this feature |

---

## ZIP Archive Layout

```
modunote-export-<timestamp>.zip
├── blocks.jsonl             # one JSON object per line (NDJSON)
├── directional_links.json   # array of [from_id, to_id] pairs
└── related_links.json       # array of [a_id, b_id] pairs
```

### `blocks.jsonl`

Each line is a self-contained JSON object representing one block.

```jsonc
// one line per block
{"id":"<uuid>","title":"Block title","content":"…","created_at":"2024-01-01T00:00:00Z","updated_at":"2024-06-15T12:34:56Z"}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID string | Stable block identifier |
| `title` | string | Block title |
| `content` | string | Block body (Markdown / rich text) |
| `created_at` | ISO 8601 UTC | Original creation timestamp |
| `updated_at` | ISO 8601 UTC | Last modification timestamp — used for conflict resolution on import |

### `directional_links.json`

An array of ordered `[from_id, to_id]` pairs representing parent → child (directional) edges.

```json
[
  ["<parent-uuid>", "<child-uuid>"],
  ["<parent-uuid>", "<child-uuid>"]
]
```

The order of each pair is significant: `from_id` is the parent (source), `to_id` is the child (target). This mirrors the `block_from_id` / `block_to_id` fields on `BlockDirectionalLink`.

### `related_links.json`

An array of unordered `[a_id, b_id]` pairs representing bidirectional "related" edges.

```json
[
  ["<uuid-a>", "<uuid-b>"],
  ["<uuid-a>", "<uuid-b>"]
]
```

The pair `[a_id, b_id]` is stored with the lower UUID first (matching the canonical ordering enforced by `BlockRelatedLink::new`), but the importer must treat both orderings as equivalent when checking for duplicates.

---

## Import Logic

### Blocks (`blocks.jsonl`)

Process each line sequentially:

1. Look up the block by `id` in the database.
2. **If the block does not exist** — insert it as a new block, preserving the original `id`, `created_at`, and `updated_at`.
3. **If the block already exists** — compare `updated_at`:
   - If the file's `updated_at` is **strictly newer** than the stored value, replace the block's `title`, `content`, and `updated_at` with the values from the file. `created_at` and `id` are never changed.
   - Otherwise, leave the existing record untouched.

### Directional links (`directional_links.json`)

For each `[from_id, to_id]` pair:

1. Check whether a directional edge between `from_id` → `to_id` already exists in the database (regardless of edge `id`).
2. **If the edge does not exist** — insert a new `BlockDirectionalLink` with a freshly generated `id`.
3. **If the edge already exists** (even with a different `id`) — skip; do not insert a duplicate.

> Both blocks referenced by an edge must already exist in the database (either pre-existing or just imported from `blocks.jsonl`). Skip or log edges whose blocks are missing.

### Related links (`related_links.json`)

For each `[a_id, b_id]` pair:

1. Normalise the pair: swap so the lower UUID is `a_id` (matching `BlockRelatedLink` canonical order).
2. Check whether a related edge between these two blocks already exists.
3. **If it does not exist** — insert a new `BlockRelatedLink`.
4. **If it already exists** — skip.

> Same missing-block guard applies as for directional links.

---

## Export Logic

The export is performed entirely on the frontend using the existing REST API:

1. Fetch all blocks (paginated if necessary).
2. Fetch all directional links (parent/child edges for each block, deduplicated).
3. Fetch all related links.
4. Serialise:
   - Blocks → `blocks.jsonl` (one JSON line per block).
   - Directional edges → `directional_links.json` (array of `[from_id, to_id]`).
   - Related edges → `related_links.json` (array of `[a_id, b_id]`).
5. Pack the three files into a ZIP archive using `fflate` (or equivalent in-browser zip library).
6. Trigger a browser download named `modunote-export-<ISO timestamp>.zip`.

---

## Conflict-Resolution Summary

| Entity | Duplicate detection | Resolution |
|--------|---------------------|------------|
| Block (same `id`, same or older `updated_at`) | `id` match | Skip |
| Block (same `id`, newer `updated_at`) | `id` match | Overwrite `title`, `content`, `updated_at` |
| Directional link | `(from_id, to_id)` pair exists | Skip |
| Related link | `{a_id, b_id}` set exists | Skip |

IDs for newly inserted links are always freshly generated on import — the edge `id` from the export file is not preserved.
