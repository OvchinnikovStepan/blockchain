use clap::{Arg, Command};
use std::sync::Arc;
use tokio::sync::Mutex;

mod api;
mod classes;

use api::run_api;
use classes::node::Node;

#[tokio::main]
async fn main() {
    let matches = Command::new("blockchain-node")
        .version("1.0")
        .author("Ovchinnikov Stepan ovstal@gmail.com")
        .about("A simple blockchain system with Proof of Work based on number of zeroes in hash beginning")
        .arg(
            Arg::new("difficulty")
                .short('d')
                .long("difficulty")
                .value_name("DIFFICULTY")
                .help("Sets the difficulty for Proof of Work")
                .default_value("4")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the port for the API")
                .default_value("3030")
                .value_parser(clap::value_parser!(u16)),
        )
        .get_matches();

    let difficulty: usize = *matches
        .get_one::<usize>("difficulty")
        .unwrap();

    let port: u16 = *matches
        .get_one::<u16>("port")
        .unwrap();

    println!("Starting node with difficulty {} on port {}", difficulty, port);

    let nodes = Arc::new(Mutex::new(Vec::<Node>::new()));

    run_api(nodes, difficulty, port).await;
}
