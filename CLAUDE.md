# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Frontend (React + TypeScript + Vite)

Navigate to `frontend/` directory for all frontend commands:

- `pnpm dev` - Start development server on port 3000
- `pnpm build` - Build for production (runs TypeScript compilation + Vite build)
- `pnpm lint` - Run ESLint
- `pnpm preview` - Preview production build

### Backend (Rust + Axum)

Navigate to root directory for backend commands:

- `cargo run -p backend` - Run backend server on port 8080
- `cargo build` - Build all workspace crates
- `cargo test` - Run tests across workspace

### Docker

- `docker-compose up` - Run full stack (backend on :8080, frontend on :3000)

## Architecture Overview

Modunote is a block-based note-taking application with two main interfaces:

###

**Blocks**: The fundamental unit of content. Each block contains:

- Markdown content that adheres to standard markdown syntax, compatible with react-markdown and other standard markdown renderers
- Support for images and files, which are stored in a dedicated space (implementation not finalized)
- Two types of links:
  - Parent-child links: Directional relationships that organize blocks into a DAG (Directed Acyclic Graph)
  - Related links: Undirectional connections between blocks for cross-referencing
- Unique ID and metadata

**Canvas**: Visual interface for spatially arranging blocks:

- Blocks can be resized and moved within the canvas
- Blocks snap to and fit within a predefined grid space
- Can toggle to show block content or just title for each block respectively
- Each block can be zoomed in/out independently, resizing title and content within the block (separate from block shape resizing)
- Parent-child links are shown by default, related links are hidden by default
- Link visibility can be toggled for each block respectively
- Canvas supports zoom in/out with all content resizing accordingly

### Backend Architecture (Rust)

Workspace-based Rust project with modular crates:

- `backend/` - Axum web server, main entry point
- `crates/storage/` - Data persistence layer

### Frontend Architecture (React + TypeScript)

**API Layer**:

- Type-safe API client with request/response types
- Proxy configuration routes `/api/*` to backend at localhost:8080
- Auto-save functionality for block updates

**Key Frontend Dependencies**:

- Milkdown for rich markdown editing with math (KaTeX) and syntax highlighting
- Tailwind CSS for styling
- React 19 with TypeScript

**Pages**:

- **BlockPage**:
  - Left sidebar shows opened block tabs with buttons to create new blocks or open search modal for existing blocks
  - Central panel is an inline block editor for markdown input with auto-save after idling (similar to Notion or Colab)
  - Right sidebar shows parents, children, and related blocks of the current active block, each section with buttons to create new blocks or search/import existing blocks and establish links; clicking items opens blocks in central panel and left sidebar; hovering items shows preview tooltip of the linked block
- **CanvasPage**:
  - Left sidebar shows pinned blocks and canvases with buttons to create new canvas, search and pin blocks, and search and pin canvases
  - Main panel displays and edits the active canvas with functionality described in Core Concepts Canvas section
  - Blocks in left sidebar are organized as a tree view showing all possible paths from ascendant to descendant blocks (non-pinned blocks in paths are ignored)
  - Blocks in left sidebar act as tags/directories for pinned canvases; pinned canvases appear under the lowest level blocks they contain or under the lowest level ascendant blocks of contained blocks
- **BlockGraph**: TODO

## Code Conventions

### Frontend

- Uses `@/` alias for `src/` imports
- Functional components with TypeScript
- Async/await pattern for API calls

### Backend

- Workspace dependencies defined in root Cargo.toml
- Modular crate architecture
- UUID-based entity IDs
- DateTime<Utc> for timestamps
