# Mini Test Server

Minimal REST API built with [Axum](https://github.com/tokio-rs/axum) for creating, listing, updating, and deleting “tests”.

Data is stored **in memory** (a `Vec<Test>` behind a `Mutex`) and will be lost when the server restarts.

## Requirements

- Rust **edition 2024** (use a recent stable Rust toolchain)

## Run

```bash
cargo run
```

Server listens on `0.0.0.0:3000`.

## Endpoints

### `GET /`

Returns basic server info.

### `GET /health`

Health check.

### `GET /tests`

Returns all tests.

### `POST /tests`

Create a new test.

Request body:

```json
{
  "title": "Midterm 1",
  "subject": "Math"
}
```

Response includes the created `id`.

### `GET /tests/{id}`

Fetch a test by id (UUID).

### `PUT /tests/{id}`

Update an existing test. Fields are optional.

Request body:

```json
{
  "title": "Midterm 1 (updated)",
  "subject": "Further Math"
}
```

### `DELETE /tests/{id}`

Delete a test by id.

## Example `curl` usage

```bash
# home
curl -s http://localhost:3000/ | jq

# health
curl -s http://localhost:3000/health | jq

# create
ID=$(
  curl -s -X POST http://localhost:3000/tests \
    -H 'content-type: application/json' \
    -d '{"title":"Midterm 1","subject":"Math"}' | jq -r .id
)
echo "$ID"

# list
curl -s http://localhost:3000/tests | jq

# get by id
curl -s "http://localhost:3000/tests/$ID" | jq

# update
curl -s -X PUT "http://localhost:3000/tests/$ID" \
  -H 'content-type: application/json' \
  -d '{"title":"Midterm 1 (updated)"}' | jq

# delete
curl -s -X DELETE "http://localhost:3000/tests/$ID" | jq
```

## Project layout

- `src/main.rs`: server bootstrap + listener
- `src/routes/mod.rs`: route definitions
- `src/handlers/mod.rs`: request handlers for `/tests`
- `src/models/mod.rs`: request/response models + `AppState`
- `src/health/mod.rs`: `GET /health` handler

## Notes

- This server is intentionally minimal: no persistence, authentication, pagination, or validation.
- If you need persistence, add a database (or serialize to a file) and replace the in-memory `Vec<Test>`.
