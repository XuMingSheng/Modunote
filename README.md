# Modunote (WIP)

Modunote is a personal knowledge management system designed for organizing and visualizing complex ideas through a flexible, graph-based architecture.

## Core Concept

Unlike traditional linear note apps, Modunote treats every note as a Block within a multi-dimensional graph:

- **Directed Links:** Notes are structured as a Directed Acyclic Graph. Links have a specific direction (parent ⮕ child), ensuring a clear hierarchy while strictly preventing circular loops.

- **Related Links:** Support for undirected connections allows you to associate blocks conceptually without implying a hierarchy.

## Visual Canvas

The primary interface is an whiteboard canvas that allows you to:

- Visualize Relationships: See the physical structure of your thoughts.
- Fluid Navigation: Smoothly pan and zoom across your selected set of blocks.
- Position notes freely to create mental maps that make sense to you.

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
