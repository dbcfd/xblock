use std::hash::Hash;

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Genesis {
    pub amt: u64,
}

impl std::hash::Hash for Genesis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.hash(self.amt);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transfer {
    pub from: u64,
    pub to: u64,
}

impl std::hash::Hash for Transfer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.hash(self.from);
        state.hash(self.to);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Script {
    Transfer(Transfer),
}

impl std::hash::Hash for Script {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Transfer(v) => v.hash(state),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Data {
    Genesis(Genesis),
    Script(Script),
}

impl std::hash::Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Genesis(v) => v.hash(state),
            Self::Script(v) => v.hash(state),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: Vec<u8>,
    pub previous_hash: Vec<u8>,
    pub timestamp: u64,
    pub data: Data,
    pub nonce: u64,
}

impl Block {
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.hash(self);
        hasher.finalize()
    }

    pub fn verify(&self) -> bool {
        let hash = self.calculate_hash();
        self.hash == hash
    }

    pub fn genesis() -> Self {
        let mut block = Block {
            id: 0,
            hash: vec![],
            previous_hash: vec![],
            timestamp: chrono::Utc::now().timestamp(),
            data: Data::Genesis(Genesis {
                amt: 1_000_000
            }),
            nonce: 42,
        };
        let hash = block.calculate_hash();
        block.hash = hash;
        block
    }

    pub fn mine(&self, data: Data, difficulty: &[u8]) -> Self {
        log::info!("mining block...");
        let mut block = Block {
            id: self.id + 1,
            hash: vec![],
            previous_hash: vec![],
            timestamp: chrono::Utc::now().timestamp(),
            data: data,
            nonce: 0,
        };
    
        loop {
            if block.nonce % 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            
            let hash = block.calculate_hash();
            if hash.starts_with(difficulty) {
                info!(
                    "mined! nonce: {}, hash: {}, binary hash: {}",
                    block.nonce,
                );
                block.hash = hash;
                return block;
            }
            block.nonce += 1;
        }
    }
}

impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(&mut hasher);
        self.previous_hash.hash(&mut hasher);
        self.timestamp.hash(&mut hasher);
        self.data.hash(&mut hasher);
        self.nonce.hash(&mut hasher);
    }
}