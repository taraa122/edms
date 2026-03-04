# EDMS — Endpoint Data Management System

EDMS is a collaborative API testing platform built in Rust. Think Postman, but multiplayer — multiple users share live endpoint state, test results stream in real time, and request/response history is persisted automatically.

## How it works

The server is built with Axum and communicates with clients over WebSockets. When you fire a test, the server spawns a separate child process to make the actual HTTP request. The child reads its assignment, executes the call, then reports the result back to the parent via an internal callback endpoint. This keeps the main server non-blocking while results broadcast to all connected clients simultaneously.

Persistent state — endpoints, bookmarks, history, collections — lives in SQLite.

## Features

- Run HTTP requests (GET, POST, PUT, DELETE, PATCH, HEAD) against saved endpoints
- Real-time test results streamed over WebSocket to all connected clients
- Bookmark endpoints into named collections
- Persistent request/response history saved to disk under `edms_data/`
- Timer tracking per request with configurable timeout and tick interval
- Dashboard showing live counts of endpoints, bookmarks, and history
- Seed data included for quick demo setup


## Getting started

**Prerequisites:** Rust 1.82+

**Build:**
```bash
cargo build
```

**Seed the database:**
```bash
sqlite3 demo.db < seed_data.sql
```

**Run:**
```bash
cargo run --bin rust-webserver
```

The backend starts on port 3000. Once it's running, open index.html directly in your browser — it connects to the backend automatically via WebSocket and HTTP.

**Or with Docker:**
```bash
docker build -t edms .
docker run -p 8080:8080 edms
```

## API overview

| Method | Route | Description |
|--------|-------|-------------|
| WS | `/test-view/run` | Run a test against an endpoint |
| WS | `/test-view/endpoints/load` | Load all endpoints |
| WS | `/test-view/bookmarks/load` | Load bookmarked endpoints |
| POST | `/test-view/save/history` | Save a history entry |
| POST | `/test-view/save/bookmark` | Save a bookmark |
| POST | `/test-view/history/clearall` | Clear all history |
| GET | `/dataview/dashboard` | Get live counts |
| POST | `/bookmarks/:collection/create` | Save active bookmarks as a collection |

## Notes

- The `edms` core library is included under `libs/edms/` as a local dependency
- `edms_data/` contains sample request/response pairs from the demo dataset
- `demo.db` and `edms.db` are included for reference but can be regenerated from `seed_data.sql`
