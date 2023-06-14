use sha3::*;
use std::fs;

pub fn calculate_sha3_224() -> String {
    let mut hasher = Sha3_224::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_sha3_256() -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_sha3_384() -> String {
    let mut hasher = Sha3_384::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_sha3_512() -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}
