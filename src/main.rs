use clap::Parser;
use std::fs::*;
use std::fs;
use std::io::*;
use std::os::unix::prelude::PermissionsExt;
use std::path::*;
use sha256;

#[derive(Parser)]
struct Cli {
    pub method: String,
    pub file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut tmp_file = File::create("xdsum.tmp").expect("Failed to create temp file");
    tmp_file.write(fs::read_to_string(args.file.clone()).expect("Failed to read input file").as_bytes()).expect("Failed to write to the temp file");

    if args.method.eq("sha256") || args.method.eq("sha256sum") {
        // get metadata
        let metadata = metadata(args.file).expect("Failed to get file metadata");

        // file mode
        tmp_file.write(&[metadata.permissions().mode() as u8]).expect("Failed to write permission nodes to the temp file");
        println!("{}", sha256::try_digest(Path::new("./xdsum.tmp")).unwrap());
    }
    
    // remove the tmp file
    fs::remove_file("xdsum.tmp").expect("Failed to remove the temp file");
}
