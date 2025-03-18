use std::io;

mod classes;

use classes::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(4);

    blockchain.add_block(String::from("Block 1 Data"));
    blockchain.add_block(String::from("Block 2 Data"));
    blockchain.add_block(String::from("Block 3 Data"));

    println!("{:#?}", blockchain);

    println!("Blockchain JSON:\n{}", blockchain.to_json());

    println!("Is chain valid? - {}",blockchain.is_chain_valid());
}
