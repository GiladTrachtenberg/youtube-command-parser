# YouTube Comment Parser

A Rust CLI tool that fetches comments from a YouTube video and searches them for a given string using the YouTube Data API v3.

## Prerequisites

- Rust (edition 2024)
- A Google Cloud project with the **YouTube Data API v3** enabled
- OAuth 2.0 credentials (Desktop/Installed application type)

## Environment Variables

| Variable               | Description                                   |
| ---------------------- | --------------------------------------------- |
| `GOOGLE_CLIENT_ID`     | OAuth 2.0 client ID from Google Cloud Console |
| `GOOGLE_CLIENT_SECRET` | OAuth 2.0 client secret                       |
| `VIDEO_ID`             | YouTube video ID to fetch comments from       |
| `SEARCH_STRING`        | The text to search for within comments        |

## Usage

```bash
export GOOGLE_CLIENT_ID="your-client-id"
export GOOGLE_CLIENT_SECRET="your-client-secret"
export VIDEO_ID="dQw4w9WgXcQ"
export SEARCH_STRING="text to find"

cargo run
```

On first run, your browser will open for Google OAuth consent. After authorization, the tool fetches top-level comments from the configured video and prints any that contain your search string.

## How It Works

1. Authenticates with Google via OAuth 2.0 installed-app flow (opens browser)
2. Fetches comment threads from a YouTube video using the Data API v3
3. Iterates through top-level comments and prints matches containing `SEARCH_STRING`

## Dependencies

- [`google-youtube3`](https://crates.io/crates/google-youtube3) — YouTube Data API v3 client
- [`tokio`](https://crates.io/crates/tokio) — Async runtime
- [`open`](https://crates.io/crates/open) — Opens URLs in the system browser
