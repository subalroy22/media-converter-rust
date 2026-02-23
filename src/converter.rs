use std::path::{Path, PathBuf};
use std::error::Error;
use std::process::Command;
use std::fs;

pub fn convert_to_mp3(input: &Path)->Result<(), Box<dyn Error>> {
    if !input.exists() {
        return Err("Input file does not exist".into());
    }

    let metadata = fs::metadata(input)?;
    
    if !metadata.is_file() {
        return Err("Input must be a file, not a directory".into());
    }

    let file_size_mb = metadata.len() as f64 / 1_048_576.0;
    println!("Input file size: {:.2} MB", file_size_mb);

    if input.extension().and_then(|ext| ext.to_str()) != Some("mp4") {
        return Err("Input file must be .mp4".into());
    }

    let output = generate_output_path(input);

    // Check if output file already exists
    if output.exists() {
        println!("Warning: Output file already exists, it will be overwritten");
        fs::remove_file(&output)?;
        println!("Removed existing file: {}", output.display());
    }

    println!("Starting conversion...");
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            &input.to_string_lossy(),
            "-vn",
            "-acodec",
            "mp3",
            &output.to_string_lossy(),
        ])
        .status()?;

    if status.success() {
        verify_output_file(&output)?;
        Ok(())
    } else {
        Err("FFmpeg conversion failed".into())
    }
}

fn verify_output_file(output: &Path) -> Result<(), Box<dyn Error>> {
    if !output.exists() {
        return Err("Output file was not created".into());
    }

    // Get metadata of the output file
    let metadata = fs::metadata(output)?;

    // Check if file has content
    if metadata.len() == 0 {
        return Err("Output file is empty".into());
    }

    Ok(())
}

pub fn generate_output_path(input: &Path)-> PathBuf {
    let mut output = input.to_path_buf();
    output.set_extension("mp3");
    output
}
