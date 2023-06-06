use clap::Parser;
use std::io::Read;
use std::ops::{Shr, Index};
use std::{fs::File, path::PathBuf};
use std::path::{self, Path};

#[derive(Parser)]
struct Cli {
    pub file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    
    let path = File::open(args.file);
    let bytes: &mut Vec<u8> = &mut vec![];
    path.unwrap().read_to_end(bytes).unwrap();

    let mut summed = bytes.clone().to_owned();
    for i in bytes.iter().enumerate() {
        if(i.0 == bytes.len() - 1) {
            break;
        }
        summed[i.0 + 1] *= i.1;
    }
    println!("{:#?}", summed);
    
    let ret: &mut Vec<char> = &mut vec![];
    let mut count = 0;
    for mut i in summed.clone() {
        i >>= count;
        count += 1;
        if(i.is_ascii_alphanumeric()) {
            ret.push(i as char);
        }
    }
    
    println!("{}", ret.into_iter().map(|i| i.to_string()).collect::<String>());
}
