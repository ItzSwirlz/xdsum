// Unless i can find some easier way to on runtime, assign a hasher variable
// a different type, the current solution is having functions for each method
// I did try if statements initializing a hasher but then it went EEEE, and
// the sha2 CoreWrapper's are pretty dense.
// This file stores all sha2 algorithms.
use sha2::*;
use std::fs;

pub fn calculate_sha224() {
    let mut hasher = Sha224::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_sha256() {
    let mut hasher = Sha256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_sha384() {
    let mut hasher = Sha384::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_sha512() {
    let mut hasher = Sha512::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_sha512_224() {
    let mut hasher = Sha512_224::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_sha512_256() {
    let mut hasher = Sha512_256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}
