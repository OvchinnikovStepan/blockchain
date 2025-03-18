use super::blockchain::Blockchain;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Node {
    pub blockchain: Blockchain,
    pub address: SocketAddr,
}

impl Node {
    pub fn new(difficulty: usize, address: SocketAddr) -> Self {
        Node {
            blockchain: Blockchain::new(difficulty),
            address,
        }
    }

    pub fn add_block(&mut self, data: String) {
        self.blockchain.add_block(data);
    }

    pub fn update_blockchain(&mut self, incoming_chain: Blockchain) {
        if incoming_chain.len() > self.blockchain.len() {
            self.blockchain = incoming_chain;
            println!("Node {}: blockchain updated",self.address)
        }
        else {
            println!("Node {}: update denyed",self.address)
        }
    }
}