use shabal::*;
use std::fs;

pub fn calculate_shabal192() -> String {
    let mut hasher = Shabal192::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_shabal224() -> String {
    let mut hasher = Shabal224::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_shabal256() -> String {
    let mut hasher = Shabal256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_shabal384() -> String {
    let mut hasher = Shabal384::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_shabal512() -> String {
    let mut hasher = Shabal512::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}
