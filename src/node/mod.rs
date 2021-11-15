use crate::blockchain::Blockchain;

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
}