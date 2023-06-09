use ascon_hash::AsconHash;
use belt_hash::BeltHash;
use blake2::{Blake2b512, Blake2s256};
use gost94::Gost94CryptoPro;
use groestl::Groestl256;
use md2::Md2;
use md4::Md4;
use md5;
use sha1::Sha1;
use sm3::Sm3;
use std::fs;
use streebog::{Streebog256, Streebog512};
use tiger::Tiger;
use whirlpool::{Digest, Whirlpool};

pub fn calculate_md5() -> String {
    let digest = md5::compute(fs::read("xdsum.tmp").unwrap());

    return format!("{:x}", digest);
}

pub fn calculate_md2() -> String {
    let mut hasher = Md2::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_md4() -> String {
    let mut hasher = Md4::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_wp() -> String {
    let mut hasher = Whirlpool::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_tiger() -> String {
    let mut hasher = Tiger::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_streebog256() -> String {
    let mut hasher = Streebog256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_streebog512() -> String {
    let mut hasher = Streebog512::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_sm3() -> String {
    let mut hasher = Sm3::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_asconhash() -> String {
    let mut hasher = AsconHash::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_belthash() -> String {
    let mut hasher = BeltHash::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_gost94() -> String {
    let mut hasher = Gost94CryptoPro::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_groestl() -> String {
    let mut hasher = Groestl256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_sha1() -> String {
    let mut hasher = Sha1::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;

}

pub fn calculate_blake2s256() -> String {
    let mut hasher = Blake2s256::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}

pub fn calculate_blake2b512() -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(fs::read("xdsum.tmp").unwrap());
    let ret = hasher.finalize().to_vec();
    let mret: String = ret.iter().map(|x| format!("{:x}", x).to_string()).collect();

    return mret;
}
