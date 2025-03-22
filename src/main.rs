mod api;
mod classes;

use std::sync::Arc;
use tokio::sync::Mutex;
use classes::node::Node;

#[tokio::main]
async fn main() {
    let nodes = Arc::new(Mutex::new(Vec::<Node>::new()));

    let port = 3030;
    let difficulty = 4;
    println!("Запуск API на http://127.0.0.1:{}", port);
    api::run_api(nodes, difficulty, port).await;
}
