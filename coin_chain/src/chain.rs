use crate::block::{Block, Data};

use serde::{Deserialize, Serialize};
use log::debug;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Chain {
    old: Vec<Block>,
    last: Block,
}

const DIFFICULTY_PREFIX: &[u8] = &[0u8, 0u8];

fn is_block_valid(block: &Block, previous: &Block) -> bool {
    if block.id != previous.id + 1 {
        debug!(
            "block with id: {} is not the next block after the latest: {}",
            block.id, previous.id
        );
        return false;
    }

    if block.previous_hash != previous.hash {
        debug!("block with id: {} has wrong previous hash", block.id);
        return false;
    }

    if !block.hash.starts_with(DIFFICULTY_PREFIX) {
        debug!("block with id: {} has invalid difficulty", block.id);
        return false;
    }

    if block.hash != block.calculate_hash() {
        debug!("block with id: {} has invalid hash", block.id);
        return false;
    }

    true
}

impl Chain {
    pub fn new() -> Self {
        let block = Block::new();
        Self {
            old: Vec::with_capacity(100),
            last: block,
        }
    }

    pub fn last(&self) -> &Block {
        &self.last
    }

    pub fn append(mut self, block: Block) -> Self {
        self.old.push(self.last);
        Self {
            old: self.old,
            last: block,
        }
    }

    pub fn mine(mut self, data: Data) -> Self {
        let block = self.last.mine(data, DIFFICULTY_PREFIX);
        self.append(block)
    }

    pub fn is_block_valid(&self, block: &Block) -> bool {
        is_block_valid(block, &self.last)
    }

    pub fn is_valid(&self) -> bool {
        let mut current_block = &self.last;
        for previous_block in self.old.iter().rev() {
            if !is_block_valid(current_block, previous_block) {
                return false;
            }
            current_block = previous_block;
        }
        true
    }

    pub fn len(&self) -> u64 {
        self.old.len() + 1
    }

    pub fn choose_chain(self, other: Self) -> Self {
        let is_local_valid = self.is_valid();
        let is_remote_valid = other.is_valid();

        if is_local_valid && is_remote_valid {
            if self.len() >= other.len() {
                self
            } else {
                other
            }
        } else if is_remote_valid && !is_local_valid {
            other
        } else if !is_remote_valid && is_local_valid {
            self
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}

