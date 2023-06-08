mod sha2algs;
use clap::Parser;
use sha2algs::*;
use std::fs;
use std::fs::*;
use std::io::*;
use std::os::unix::prelude::PermissionsExt;
use std::u8;

#[derive(Parser)]
struct Cli {
    pub function: String,
    pub file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut tmp_file = File::create("xdsum.tmp").expect("Failed to create temp file");
    tmp_file
        .write(
            fs::read_to_string(args.file.clone())
                .expect("Failed to read input file")
                .as_bytes(),
        )
        .expect("Failed to write to the temp file");

    // get metadata
    let metadata = metadata(args.file).expect("Failed to get file metadata");

    // file mode
    tmp_file
        .write(&[metadata.permissions().mode() as u8])
        .expect("Failed to write permission nodes to the temp file");

    match args.function.to_lowercase().as_str() {
        "sha224sum" | "sha224" => calculate_sha224(),
        "sha256sum" | "sha256" => calculate_sha256(),
        "sha384sum" | "sha384" => calculate_sha384(),
        "sha512sum" | "sha512" => calculate_sha512(),
        "sha512sum_224" | "sha512_224" | "sha512_224sum" => calculate_sha512_224(),
        "sha512sum_256" | "sha512_256" | "sha512_256sum" => calculate_sha512_256(),
        _ => {
            println!(
                "{} is not a valid cryptographic hash function",
                args.function
            );
        }
    }

    // remove the tmp file
    fs::remove_file("xdsum.tmp").expect("Failed to remove the temp file");
}
