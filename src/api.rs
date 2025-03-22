use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::classes::node::Node;
use crate::classes::block::Block;
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    message: String,
    status_code: u16,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(message: String, status_code: u16, data: Option<T>) -> Self {
        ApiResponse {
            success: true,
            message,
            status_code,
            data,
        }
    }

    fn error(message: String, status_code: u16) -> Self {
        ApiResponse {
            success: false,
            message,
            status_code,
            data: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct AddBlockRequest {
    data: String,
}

fn create_node(
    nodes: Arc<Mutex<Vec<Node>>>,
    difficulty: usize,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("node" / "create")
        .and(warp::post())
        .and_then(move || {
            let nodes = nodes.clone();
            async move {
                let mut nodes = nodes.lock().await;
                let address = format!("127.0.0.1:{}", 3030 + nodes.len());
                let node = Node::new(difficulty, address.parse().unwrap());
                nodes.push(node.clone());

                let response: ApiResponse<String> = ApiResponse::success(
                    format!("Node created: {}", node.get_address()),
                    StatusCode::CREATED.as_u16(),
                    Some(node.get_address()),
                );
                Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&response), StatusCode::CREATED))
            }
        })
}

fn get_chain(
    nodes: Arc<Mutex<Vec<Node>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("node" / String / "chain")
        .and(warp::get())
        .and_then(move |address: String| {
            let nodes = nodes.clone();
            async move {
                let nodes = nodes.lock().await;
                if let Some(node) = nodes.iter().find(|n| n.get_address() == address) {
                    let response: ApiResponse<&Vec<Block>> = ApiResponse::success(
                        "Blockchain received".to_string(),
                        StatusCode::OK.as_u16(),
                        Some(&node.blockchain.chain),
                    );
                    Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&response), StatusCode::OK))
                } else {
                    let response: ApiResponse<()> = ApiResponse::error(
                        "Node is not found".to_string(),
                        StatusCode::NOT_FOUND.as_u16(),
                    );
                    Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&response), StatusCode::NOT_FOUND))
                }
            }
        })
}

pub async fn run_api(nodes: Arc<Mutex<Vec<Node>>>, difficulty: usize, port: u16) {
    let create_node_route = create_node(Arc::clone(&nodes), difficulty);
    let get_chain_route = get_chain(Arc::clone(&nodes));

    let routes = create_node_route
        .or(get_chain_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}
