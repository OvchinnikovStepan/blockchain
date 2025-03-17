use super::block::Block;

#[derive(Debug)]
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
}
