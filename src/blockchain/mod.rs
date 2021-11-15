use chrono::{NaiveDateTime, Utc};
use sha2::{Digest, Sha256};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blocks = Vec::<Block>::new();
        blocks.push(Block::genesis());
        Blockchain { blocks }
    }

    pub fn check_integrity(blocks: &Vec<Block>) -> Result<(), (usize, IntegrityViolation)> {
        let mut previous_block: Option<&Block> = None;
        for (index, block) in blocks.iter().enumerate() {
            match block.check_integrity(previous_block) {
                Err(violation) => return Err((index, violation)),
                Ok(_) => (),
            }
            previous_block = Some(block);
        }
        Ok(())
    }

    pub fn replace_chain(&mut self, new_blocks: &Vec<Block>) -> Result<(), ()> {
        if Self::check_integrity(new_blocks).is_ok() && new_blocks.len() > self.blocks.len() {
            self.blocks = new_blocks.clone();
            Ok(())
        } else {
            Err(())
        }
    }
}

pub enum IntegrityViolation {
    InvalidIndex,
    InvalidGenesis,
    InvalidPreviousHash,
    InvalidHash,
}

#[derive(Clone)]
pub struct Block {
    pub index: usize,
    pub hash: Option<String>,
    pub previous_hash: Option<String>,
    pub timestamp: NaiveDateTime,
    pub data: String,
}

impl Block {
    pub fn genesis() -> Self {
        let mut genesis_block = Block {
            index: 0,
            hash: None,
            previous_hash: None,
            timestamp: Utc::now().naive_utc(),
            data: Self::genesis_data(),
        };
        genesis_block.set_hash();
        genesis_block
    }

    fn genesis_data() -> String {
        "Neon genesis!".to_owned()
    }

    pub fn generate_next(&self, data: String) -> Self {
        let mut next_block = Block {
            index: self.index + 1,
            hash: None,
            previous_hash: self.hash.clone(),
            timestamp: Utc::now().naive_utc(),
            data,
        };
        next_block.set_hash();
        next_block
    }

    pub fn set_hash(&mut self) {
        self.hash = Some(self.calculate_hash());
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{:?}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data
        ));
        format!("{:X}", hasher.finalize())
    }

    pub fn check_integrity(&self, previous_block: Option<&Self>) -> Result<(), IntegrityViolation> {
        match previous_block {
            Some(previous_block) => {
                if self.index != previous_block.index + 1 {
                    return Err(IntegrityViolation::InvalidIndex);
                }
                if self
                    .previous_hash
                    .ne(&Some(previous_block.calculate_hash()))
                {
                    return Err(IntegrityViolation::InvalidPreviousHash);
                }
            }
            None => {
                if self.index != 0 {
                    return Err(IntegrityViolation::InvalidIndex);
                }
                if self.previous_hash.ne(&None) {
                    return Err(IntegrityViolation::InvalidPreviousHash);
                }
                if self.data.ne(&Self::genesis_data()) {
                    return Err(IntegrityViolation::InvalidGenesis);
                }
            }
        }
        if self.hash.ne(&Some(self.calculate_hash())) {
            return Err(IntegrityViolation::InvalidHash);
        }
        Ok(())
    }
}
