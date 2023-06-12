use jh::*;
use std::fs;

pub fn calculate_jh224() {
    let mut hasher = Jh224::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_jh256() {
    let mut hasher = Jh256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_jh384() {
    let mut hasher = Jh384::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}

pub fn calculate_jh512() {
    let mut hasher = Jh512::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    println!("{}", mret);
}
