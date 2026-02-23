mod converter;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "MP4 to MP3 converter")]
struct Args {
    input: PathBuf
}

fn main() {
    let args = Args::parse();

    match converter::convert_to_mp3(&args.input) {
        Ok(_) => println!("Conversion successfull"),
        Err(e) => eprintln!("Failed: {}", e)
    }
}