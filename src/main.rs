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
    let permissions = metadata.permissions();

    // file mode
    tmp_file
        .write(&[permissions.mode() as u8])
        .expect("Failed to write permission nodes to the temp file");

    tmp_file
        .write(&[permissions.readonly() as u8])
        .expect("Failed to write readonly vale to the temp file");

    match args.function.to_lowercase().as_str() {
        "sha1sum" | "sha1" => calculate_sha1(),
        "sha224sum" | "sha224" => calculate_sha224(),
        "sha256sum" | "sha256" => calculate_sha256(),
        "sha384sum" | "sha384" => calculate_sha384(),
        "sha512sum" | "sha512" => calculate_sha512(),
        "sha512sum_224" | "sha512_224" | "sha512_224sum" => calculate_sha512_224(),
        "sha512sum_256" | "sha512_256" | "sha512_256sum" => calculate_sha512_256(),
        "sha3_224sum" | "sha3_224" => calculate_sha3_224(),
        "sha3_256sum" | "sha3_256" => calculate_sha3_256(),
        "sha3_384sum" | "sha3_384" => calculate_sha3_384(),
        "sha3_512sum" | "sha3_512" => calculate_sha3_512(),
        "shabal192sum" | "shabal192" => calculate_shabal192(),
        "shabal224sum" | "shabal224" => calculate_shabal224(),
        "shabal256sum" | "shabal256" => calculate_shabal256(),
        "shabal384sum" | "shabal384" => calculate_shabal384(),
        "shabal512sum" | "shabal512" => calculate_shabal512(),
        "fsb192sum" | "fsb160" => calculate_fsb160(),
        "fsb224sum" | "fsb224" => calculate_fsb224(),
        "fsb256sum" | "fsb256" => calculate_fsb256(),
        "fsb384sum" | "fsb384" => calculate_fsb384(),
        "fsb512sum" | "fsb512" => calculate_fsb512(),
        "jh224sum" | "jh224" => calculate_jh224(),
        "jh256sum" | "jh256" => calculate_jh256(),
        "jh384sum" | "jh384" => calculate_jh384(),
        "jh512sum" | "jh512" => calculate_jh512(),
        "ripemd128sum" | "ripemd128" => calculate_ripemd128(),
        "ripemd256sum" | "ripemd256" => calculate_ripemd256(),
        "ripemd160sum" | "ripemd160" => calculate_ripemd160(),
        "ripemd320sum" | "ripemd320" => calculate_ripemd320(),
        "md5sum" | "md5" => calculate_md5(),
        "md2sum" | "md2" => calculate_md2(),
        "md4sum" | "md4" => calculate_md4(),
        "whirlpoolsum" | "whirlpool" => calculate_wp(),
        "tigersum" | "tiger" => calculate_tiger(),
        "streebog256sum" | "streebog256" => calculate_streebog256(),
        "streebog512sum" | "streebog512" => calculate_streebog512(),
        "sm3sum" | "sm3" => calculate_sm3(),
        "asconsum" | "ascon" => calculate_asconhash(),
        "beltsum" | "belt" => calculate_belthash(),
        "gost94sum" | "gost94" => calculate_gost94(),
        "groestlsum" | "groestl256sum" | "groestl" | "groestl256" => calculate_groestl(),
        "blake2b512sum" | "blake2b512" => calculate_blake2b512(),
        "blake2s256sum" | "blake2s256" => calculate_blake2s256(),
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
