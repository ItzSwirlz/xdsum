mod fsb_algs;
mod gen_algs;
mod jh_algs;
mod ripemd_algs;
mod sha2algs;
mod sha3algs;
mod shabal_algs;
use clap::Parser;
use fsb_algs::*;
use gen_algs::*;
use jh_algs::*;
use ripemd_algs::*;
use sha2algs::*;
use sha3algs::*;
use shabal_algs::*;
use std::fs;
use std::fs::*;
use std::io::*;
use std::time::{UNIX_EPOCH};
use std::u8;

#[cfg(target_os = "linux")]
use std::os::unix::prelude::PermissioWnsExt;

#[derive(Parser)]
struct Cli {
    //Type of algorithm you want to use
    pub function: String,
    //File path
    pub file: std::path::PathBuf,
    //Hash time
    #[arg(short = 't', required = false)]
    pub time: bool,
    //Compare hash
    #[arg(short = 'c', required = false, default_value="")]
    //Hash to compare
    pub comp: String,
}


fn main() {
    let mut milliseconds: u128 = 0;
    let args: Cli = Cli::parse();
    let mut crypthash: String = String::from("");

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
    let permissions = metadata.permissions();

    // file length
    tmp_file
        .write(&[metadata.len() as u8])
        .expect("Failed to write length to the temp file");

    // file mode
    #[cfg(target_os = "linux")]
    tmp_file
        .write(&[permissions.mode() as u8])
        .expect("Failed to write permission nodes to the temp file");

    // read only
    tmp_file
        .write(&[permissions.readonly() as u8])
        .expect("Failed to write readonly value to the temp file");

    if args.time {
        if let Ok(time) = metadata.modified() {
            let duration = time.duration_since(UNIX_EPOCH).expect("Time went backwards");
            milliseconds = duration.as_millis();
            tmp_file
                .write(&[milliseconds as u8]);
        } else {
            println!("Time is not supported on this platform!");
        }
    }

    match args.function.to_lowercase().as_str() {
        "sha1sum" | "sha1" => crypthash = calculate_sha1(),
        "sha224sum" | "sha224" => crypthash = calculate_sha224(),
        "sha256sum" | "sha256" => crypthash = calculate_sha256(),
        "sha384sum" | "sha384" => crypthash = calculate_sha384(),
        "sha512sum" | "sha512" => crypthash = calculate_sha512(),
        "sha512sum_224" | "sha512_224" | "sha512_224sum" => crypthash = calculate_sha512_224(),
        "sha512sum_256" | "sha512_256" | "sha512_256sum" => crypthash = calculate_sha512_256(),
        "sha3_224sum" | "sha3_224" => crypthash = calculate_sha3_224(),
        "sha3_256sum" | "sha3_256" => crypthash = calculate_sha3_256(),
        "sha3_384sum" | "sha3_384" => crypthash = calculate_sha3_384(),
        "sha3_512sum" | "sha3_512" => crypthash = calculate_sha3_512(),
        "shabal192sum" | "shabal192" => crypthash = calculate_shabal192(),
        "shabal224sum" | "shabal224" => crypthash = calculate_shabal224(),
        "shabal256sum" | "shabal256" => crypthash = calculate_shabal256(),
        "shabal384sum" | "shabal384" => crypthash = calculate_shabal384(),
        "shabal512sum" | "shabal512" => crypthash = calculate_shabal512(),
        "fsb192sum" | "fsb160" => crypthash = calculate_fsb160(),
        "fsb224sum" | "fsb224" => crypthash = calculate_fsb224(),
        "fsb256sum" | "fsb256" => crypthash = calculate_fsb256(),
        "fsb384sum" | "fsb384" => crypthash = calculate_fsb384(),
        "fsb512sum" | "fsb512" => crypthash = calculate_fsb512(),
        "jh224sum" | "jh224" => crypthash = calculate_jh224(),
        "jh256sum" | "jh256" => crypthash = calculate_jh256(),
        "jh384sum" | "jh384" => crypthash = calculate_jh384(),
        "jh512sum" | "jh512" => crypthash = calculate_jh512(),
        "ripemd128sum" | "ripemd128" => crypthash = calculate_ripemd128(),
        "ripemd256sum" | "ripemd256" => crypthash = calculate_ripemd256(),
        "ripemd160sum" | "ripemd160" => crypthash = calculate_ripemd160(),
        "ripemd320sum" | "ripemd320" => crypthash = calculate_ripemd320(),
        "md5sum" | "md5" => crypthash = calculate_md5(),
        "md2sum" | "md2" => crypthash = calculate_md2(),
        "md4sum" | "md4" => crypthash = calculate_md4(),
        "whirlpoolsum" | "whirlpool" => crypthash = calculate_wp(),
        "tigersum" | "tiger" => crypthash = calculate_tiger(),
        "streebog256sum" | "streebog256" => crypthash = calculate_streebog256(),
        "streebog512sum" | "streebog512" => crypthash = calculate_streebog512(),
        "sm3sum" | "sm3" => crypthash = calculate_sm3(),
        "asconsum" | "ascon" => crypthash = calculate_asconhash(),
        "beltsum" | "belt" => crypthash = calculate_belthash(),
        "gost94sum" | "gost94" => crypthash = calculate_gost94(),
        "groestlsum" | "groestl256sum" | "groestl" | "groestl256" => crypthash = calculate_groestl(),
        "blake2b512sum" | "blake2b512" => crypthash = calculate_blake2b512(),
        "blake2s256sum" | "blake2s256" => crypthash = calculate_blake2s256(),
        _ => {
            println!(
                "{} is not a valid cryptographic hash function",
                args.function
            );
        }
    }
    
    println!("{}", crypthash);
    println!("\nxdsum noted the following extra information and added it to the hashed data:");
    println!("File Length: {}", metadata.len());

    #[cfg(target_os = "linux")]
    println!("File Mode: {}", permissions.mode());

    println!("Read Only: {}", permissions.readonly());
    if args.time {
        if let Ok(_time) = metadata.modified() {
            println!("Time Last Modified: {milliseconds:?}");
        } else {
            println!("Time was not supported on this platform supported on this platform, so time was not added to the hashed data.");
        } 
    }

    if args.comp.chars().count() > 0 {
        if args.comp == crypthash {
            println!("Hashes match.");
        } else {
            println!("Hashes do not match.");
        }
    }

    // remove the tmp file
    fs::remove_file("xdsum.tmp").expect("Failed to remove the temp file");
}
