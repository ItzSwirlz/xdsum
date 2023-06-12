use ripemd::*;
use std::fs;

pub fn calculate_ripemd160() {
    let mut hasher = Ripemd160::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_ripemd320() {
    let mut hasher = Ripemd320::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_ripemd256() {
    let mut hasher = Ripemd256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_ripemd128() {
    let mut hasher = Ripemd128::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}