use std::path::{Path, PathBuf};
use std::error::Error;
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::progress::ProgressTracker;

pub async fn convert_to_mp3_with_progress<F>(input: &Path, mut progress_callback: F)->Result<PathBuf, Box<dyn Error + Send + Sync>> where F:FnMut(f32, String) +Send + 'static,{
    if !input.exists() {
        return Err("Input file does not exist".into());
    }

    if input.extension().and_then(|ext| ext.to_str()) != Some("mp4") {
        return Err("Input file must be .mp4".into());
    }

    let output = generate_output_path(input);

    // Check if output file already exists
    if output.exists() {
        tokio::fs::remove_file(&output).await?;
    }

    progress_callback(0.0, "Starting conversion...".to_string());

    let mut child = Command::new("ffmpeg")
        .args([
            "-i",
            &input.to_string_lossy(),
            "-vn",
            "-acodec",
            "mp3",
            "-progress",
            "pipe:2",
            &output.to_string_lossy(),
        ])
        .stderr(Stdio::piped())
        .spawn()?;

    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
    let reader = BufReader::new(stderr);
    let mut lines = reader.lines();

    let tracker = ProgressTracker::new();

    while let Ok(Some(line)) = lines.next_line().await {
        if let Some(duration) = ProgressTracker::parse_duration(&line) {
            let mut total = tracker.total_duration.lock().await;
            *total = Some(duration);
        }

        if let Some(current_time) = ProgressTracker::parse_time(&line) {
            let progress = tracker.calculate_progress(current_time).await;
            progress_callback(progress, format!("Converting... {:.1}%", progress));
        }
    }

    let status = child.wait().await?;

    if status.success() {
        progress_callback(100.0, "Conversion complete".to_string());
        Ok(output)
    } else {
        Err("FFmpeg conversion failed".into())
    }
}

pub fn generate_output_path(input: &Path)-> PathBuf {
    let mut output = input.to_path_buf();
    output.set_extension("mp3");
    output
}
