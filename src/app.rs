use alloc::{string::String, vec::Vec};
use cortex_m_semihosting::hprintln;
use hex::ToHex;

use crate::block::{self, hash_to_base_representation, Block};

pub struct App {
    pub blocks: Vec<Block>,
}

impl App {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: String::from("7986b271f19b02add6d9744d48cde7dd747f3ea9"),
        };
        self.blocks.push(genesis_block);

        hprintln!("[INFO] added genesis block");
    }

    pub fn try_add_block(&mut self, block: Block) {
        let latest_block = self
            .blocks
            .last()
            .expect("[ERROR] latest block doesn't exist");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            hprintln!("[ERROR] could not add block - invalid");
        }
    }

    pub fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            hprintln!("[WARN] block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !check_difficulty(block) {
            hprintln!("[WARN] block with id: {} has invalid difficulty", block.id);
            return false;
        } else if !check_hash(block) {
            hprintln!("[WARN] block with id: {} has invalid hash", block.id);
            return false;
        }

        true
    }

    pub fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("[ERROR] first block doesn't exist");
            let second = chain.get(i).expect("[ERROR] second block doesn't exist");
            if !self.is_block_valid(second, first) {
                return false;
            }
        }

        true
    }

    /// we always choose the longest valid chain
    pub fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("[ERROR] local and remote chains are both invalid!");
        }
    }
}

fn check_difficulty(block: &Block) -> bool {
    let mut buffer = [0u8; 20];
    hex::decode_to_slice(&block.hash, &mut buffer).expect("[ERROR] failed to decode block hash");

    hash_to_base_representation(&buffer, 2).starts_with(block::DIFFICULTY_PREFIX)
}

fn check_hash(block: &Block) -> bool {
    let hash = block::calculate_hash(block.id, &block.previous_hash, &block.data, block.nonce);

    hash.as_slice().encode_hex::<String>() == block.hash
}
