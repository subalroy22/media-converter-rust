# Media Converter

A simple command-line tool to convert MP4 video files to MP3 audio files using Rust and FFmpeg.

## Prerequisites

- **Rust** (1.70+) - Install from [rustup.rs](https://rustup.rs/)
- **FFmpeg** - Must be installed and in your PATH
  - Windows: Download from [ffmpeg.org](https://ffmpeg.org/download.html)
  - macOS: `brew install ffmpeg`
  - Linux: `sudo apt install ffmpeg`

## Installation

```bash
git clone <your-repo-url>
cd media-converter-rust
cargo build --release
```

## Usage

Convert an MP4 file to MP3:

```bash
cargo run -- video.mp4
```

This creates `video.mp3` in the same directory.

## Example Output

```
Input file size: 11.28 MB
Starting conversion...
Conversion successful
```

## What It Does

1. Validates the input file exists and is an MP4
2. Shows input file size
3. Warns if output file already exists and removes it
4. Converts video to MP3 using FFmpeg (audio only)
5. Verifies the output file was created successfully

## Project Structure

```
media-converter-rust/
├── src/
│   ├── main.rs        # CLI entry point
│   └── converter.rs   # Conversion logic
└── Cargo.toml         # Dependencies
```
