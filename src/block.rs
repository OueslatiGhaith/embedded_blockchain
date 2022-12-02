use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use cortex_m_semihosting::{hprint, hprintln};
use hex::ToHex;
use numtoa::NumToA;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use sha1::{Digest, Sha1};

pub const DIFFICULTY_PREFIX: &str = "0";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    // pub timestamp: i64, // NOTE: is this possible in embedded env?
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let (nonce, hash) = mine_block(id, &previous_hash, &data);

        Self {
            id,
            hash,
            previous_hash,
            data,
            nonce,
        }
    }
}

pub fn mine_block(id: u64, previous_hash: &str, data: &str) -> (u64, String) {
    hprintln!("[INFO] mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 1 == 1_000 {
            hprintln!("[INFO] nonce: {}", nonce);
        }
        let hash = calculate_hash(id, previous_hash, data, nonce);
        let binary_hash = hash_to_base_representation(&hash, 2);
        let hex_hash = hash.as_slice().encode_hex();

        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            hprintln!(
                "[INFO] mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex_hash,
                binary_hash
            );
            return (nonce, hex_hash);
        }
        nonce += 1;
    }
}

pub fn hash_to_base_representation(hash: &[u8], base: u8) -> String {
    let mut res = String::default();
    let mut buf = [0u8; 200];
    for c in hash {
        let s = c.numtoa_str(base, &mut buf);
        res.push_str(s);
    }
    res
}

#[derive(serde::Serialize)]
struct SerializableData {
    data: String,
    id: u64,
    nonce: u64,
    previous_hash: String,
}

pub fn calculate_hash(id: u64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data: serde_json_core::heapless::String<256> =
        serde_json_core::ser::to_string(&SerializableData {
            id: id,
            previous_hash: previous_hash.to_string(),
            data: data.to_string(),
            nonce: nonce,
        })
        .unwrap();

    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    hasher.finalize().as_slice().to_owned()
}
