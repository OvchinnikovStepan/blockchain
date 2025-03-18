use std::io;

mod classes;

use classes::blockchain::Blockchain;

fn main() {
    println!("Enter Proof of Work difficulty:");

    let mut pow_difficulty = String::new();
    io::stdin()
        .read_line(&mut pow_difficulty)
        .expect("Unable to read a string");

    let pow_difficulty: usize = match pow_difficulty.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: difficulty must be a number");
            return;
        }
    };

    let mut blockchain = Blockchain::new(pow_difficulty);

    blockchain.add_block(String::from("Block 1 Data"));
    blockchain.add_block(String::from("Block 2 Data"));
    blockchain.add_block(String::from("Block 3 Data"));

    println!("{:#?}", blockchain);

    println!("Blockchain JSON:\n{}", blockchain.to_json());

    println!("Is chain valid? - {}",blockchain.is_chain_valid());
}
