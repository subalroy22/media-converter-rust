# Media Converter

A web-based media converter built with Rust and Actix-web that converts MP4 video files to MP3 audio files with real-time progress tracking via WebSockets.

## Features

- 🎵 Convert MP4 videos to MP3 audio files
- 📊 Real-time conversion progress tracking
- 🌐 Modern web interface with drag-and-drop support
- ⚡ Asynchronous processing with Tokio
- 🔌 WebSocket-based live updates
- 📥 Direct download of converted files

## Prerequisites

- **Rust** (1.70+) - Install from [rustup.rs](https://rustup.rs/)
- **FFmpeg** - Must be installed and in your PATH
  - Windows: Download from [ffmpeg.org](https://ffmpeg.org/download.html)
  - macOS: `brew install ffmpeg`
  - Linux: `sudo apt install ffmpeg`

## Installation

```bash
git clone https://github.com/subalroy22/media-converter-rust.git
cd media-converter-rust
cargo build
```

## Usage

1. Start the server:

```bash
cargo run
```

2. Open your browser and navigate to:

```
http://localhost:8080
```

3. Upload an MP4 file using the web interface (drag-and-drop or click to browse)

4. Watch the real-time conversion progress

5. Download your converted MP3 file when complete

## API Endpoints

- `POST /api/upload` - Upload an MP4 file for conversion
- `GET /api/ws/{session_id}` - WebSocket endpoint for real-time progress updates
- `GET /api/download/{session_id}` - Download the converted MP3 file
- `GET /` - Serve the web interface

## How It Works

1. User uploads an MP4 file via the web interface
2. Server generates a unique session ID and stores the file
3. WebSocket connection is established for progress tracking
4. FFmpeg converts the video to MP3 with progress monitoring
5. Real-time progress updates are sent via WebSocket
6. User can download the converted MP3 file

## Project Structure

```
media-converter-rust/
├── src/
│   ├── main.rs        # Server entry point and configuration
│   ├── routes.rs      # API route definitions
│   ├── handlers.rs    # Request handlers (upload, download, WebSocket)
│   ├── converter.rs   # FFmpeg conversion logic with progress tracking
│   ├── websocket.rs   # WebSocket handler for real-time updates
│   ├── progress.rs    # Progress calculation and parsing
│   └── models.rs      # Data models (responses, messages)
├── static/
│   ├── index.html     # Web interface
│   └── app.js         # Frontend JavaScript
├── uploads/           # Temporary storage for uploaded and converted files
└── Cargo.toml         # Dependencies
```

## Dependencies

- `actix-web` - Web framework
- `actix-ws` - WebSocket support
- `actix-files` - Static file serving
- `actix-multipart` - File upload handling
- `tokio` - Async runtime
- `serde` & `serde_json` - Serialization
- `uuid` - Unique session ID generation
- `futures-util` - Async utilities
- `regex` - FFmpeg output parsing
