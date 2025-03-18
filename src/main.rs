use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod classes;

use classes::blockchain::Blockchain;
use classes::node::Node;

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

    let mut node1 = Node::new(pow_difficulty, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    let mut node2 = Node::new(pow_difficulty, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081));

    node1.update_blockchain(blockchain);
    node2.add_block(String::from("Some Block Data"));
    node1.update_blockchain(node2.blockchain.clone());
    node2.update_blockchain(node1.blockchain.clone());

    println!("{:#?}",node2)
}
