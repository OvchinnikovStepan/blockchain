use super::block::Block;

#[derive(Clone,Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pow_difficulty: usize,
}

impl Blockchain {
    pub fn new(pow_difficulty: usize) -> Self {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
            pow_difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();
        let mut new_block = Block::new(previous_block.index + 1, previous_block.hash, data);
        new_block.mine_block(self.pow_difficulty);
        self.chain.push(new_block);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.chain).unwrap()
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn is_chain_valid(&self)->bool {
        for i in 1..self.chain.len() {
            let current_block=&self.chain[i];
            let previous_block = &self.chain[i-1];

            if current_block.hash != current_block.calculate_hash() {
                println!("Invalid hash for Block {}",current_block.index);
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                println!("Invalid previous hash for Block {}",current_block.index);
                return false;
            }
        }
        true
    }
}
