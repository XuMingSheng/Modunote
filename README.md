# Modunote (WIP)

Modunote is a personal note organizing and visualization project.

## Development Commands

### Frontend (React + TypeScript + Vite)

Run from `frontend/`:

- `pnpm dev` - Start development server on port 3000
- `pnpm build` - Build for production (TypeScript + Vite)
- `pnpm lint` - Run ESLint
- `pnpm preview` - Preview production build

### Backend (Rust + Axum)

Run from `backend/` (uses `just`):

- `just build` - Run `sqlx prepare` for both DB crates and compile
- `just test-sqlite` - Run sqlite tests with a temporary test DB
- `just test-postgres` - Run postgres tests with a temporary test container

### Docker

Run from repo root (includes backend postgres via compose override):

- `docker compose -f docker-compose.yml -f backend/docker-compose.yml up` - Run full stack (backend on :8080, frontend on :3000)

## Architecture Overview

Modunote is a block-based note-taking application with two main interfaces:

### Core Concepts

Blocks are the fundamental unit of content. Each block contains:

- Markdown content (no custom syntax for portability)
- Bidirectional links (parent/child/related blocks)
- Unique ID and metadata

Workspaces are the traditional editor interface for working with blocks linearly.

Canvas is a visual interface for spatially arranging blocks.
