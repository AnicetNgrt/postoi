use serde::Serialize;

use crate::blockchain::{Block, Blockchain};

#[derive(Serialize)]
pub struct Node {
    pub peers: Vec<String>,
    pub blockchain: Blockchain,
}

impl Node {
    pub fn init() -> Self {
        Node {
            peers: vec![],
            blockchain: Blockchain::new()
        }
    }

    pub fn mint(&mut self, data: String) -> &Block {
        self.blockchain.mint_next_block(data)
    }

    pub fn replace_chain(&mut self, new_blocks: &Vec<Block>) -> Result<(), ()> {
        self.blockchain.replace_chain(new_blocks).map(|_| self.broadcast_latest())
    }

    pub fn connect_to_peer(&mut self, peer: String) {
        self.peers.push(peer);
    }

    fn broadcast_latest(&self) {}
}