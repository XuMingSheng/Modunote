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

### Core Concepts

**Blocks**: The fundamental unit of content. Each block contains:
- Markdown content with custom syntax support
- Bidirectional links (parent/child/related blocks)
- Unique ID and metadata

**Workspaces**: Traditional editor interface for working with blocks linearly
**Canvas**: Visual interface for spatially arranging blocks

### Backend Architecture (Rust)

Workspace-based Rust project with modular crates:

- `backend/` - Axum web server, main entry point
- `crates/engine/` - Core data structures (Block, View, Asset)
- `crates/parser/` - Markdown parsing with custom syntax
- `crates/search/` - Block search and indexing
- `crates/storage/` - Data persistence layer

### Frontend Architecture (React + TypeScript)

Key architectural patterns:

**Context-based State Management**:
- `BlocksContext` - Manages open blocks, active block, and block operations
- `CanvasContext` - Canvas state management (currently stubbed)

**Two Main Views**:
- `Workplace` - Traditional editor with three-panel layout:
  - Left: Open blocks sidebar
  - Center: Active block editor (Milkdown-based markdown editor)
  - Right: Linked blocks sidebar
- `CanvasPage` - Visual block arrangement interface

**API Layer**:
- Type-safe API client with request/response types
- Proxy configuration routes `/api/*` to backend at localhost:8080
- Auto-save functionality for block updates

**Key Frontend Dependencies**:
- Milkdown for rich markdown editing with math (KaTeX) and syntax highlighting
- Tailwind CSS for styling
- React 19 with TypeScript

## Code Conventions

### Frontend
- Uses `@/` alias for `src/` imports
- Functional components with TypeScript
- Context providers for state management
- Async/await pattern for API calls

### Backend
- Workspace dependencies defined in root Cargo.toml
- Modular crate architecture
- UUID-based entity IDs
- DateTime<Utc> for timestamps