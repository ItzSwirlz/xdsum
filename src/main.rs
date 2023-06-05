use clap::Parser;
use std::io::Read;
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
    
    // now, split our file into 8 chunks
    let mut iter = bytes.chunks(bytes.len() / 8);
    while !iter.next().is_none() {
        let mut current_iter = iter.next().unwrap().to_owned();
        let mut i = 1;
        while i <= current_iter.len() - 1 {
            current_iter[i] *= (current_iter[i - 1] % 255) ;
            i += 1;
        }
        iter.next();
    }

    
    let ret: &mut Vec<char> = &mut vec![];
    for i in bytes {
        let j = *i as char;
        if(j.is_ascii_alphanumeric()) {
            ret.push(j);
        }
    }
    println!("{}", ret.into_iter().map(|i| i.to_string()).collect::<String>());
}
