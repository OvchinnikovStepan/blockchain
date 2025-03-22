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
    port: u16,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("node" / "create")
        .and(warp::post())
        .and_then(move || {
            let nodes = nodes.clone();
            async move {
                let mut nodes = nodes.lock().await;
                let address = format!("127.0.0.1:{}", port + nodes.len() as u16);
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

fn add_block(
    nodes: Arc<Mutex<Vec<Node>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("node" / String / "block")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |address: String, request: AddBlockRequest| {
            let nodes = nodes.clone();
            async move {
                let mut nodes = nodes.lock().await;
                if let Some(node) = nodes.iter_mut().find(|n| n.get_address() == address) {
                    node.add_block(request.data);
                    let response: ApiResponse<&Vec<Block>> = ApiResponse::success(
                        "Block added".to_string(),
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

fn sync_nodes(
    nodes: Arc<Mutex<Vec<Node>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("node" / String / "sync" / String)
        .and(warp::post())
        .and_then(move |from_address: String, to_address: String| {
            let nodes = nodes.clone();
            async move {
                let mut nodes = nodes.lock().await;

                let incoming_chain = nodes
                    .iter()
                    .find(|n| n.get_address() == from_address)
                    .map(|node| node.blockchain.clone());

                if let Some(incoming_chain) = incoming_chain {
                    if let Some(to_node) = nodes.iter_mut().find(|n| n.get_address() == to_address) {
                        to_node.update_blockchain(incoming_chain);
                        let response: ApiResponse<&Vec<Block>> = ApiResponse::success(
                            "Chain synchronized".to_string(),
                            StatusCode::OK.as_u16(),
                            Some(&to_node.blockchain.chain),
                        );
                        Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&response), StatusCode::OK))
                    } else {
                        let response: ApiResponse<()> = ApiResponse::error(
                            "Target node is not found".to_string(),
                            StatusCode::NOT_FOUND.as_u16(),
                        );
                        Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&response), StatusCode::NOT_FOUND))
                    }
                } else {
                    let response: ApiResponse<()> = ApiResponse::error(
                        "Source node is not found".to_string(),
                        StatusCode::NOT_FOUND.as_u16(),
                    );
                    Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&response), StatusCode::NOT_FOUND))
                }
            }
        })
}

pub async fn run_api(nodes: Arc<Mutex<Vec<Node>>>, difficulty: usize, port: u16) {
    let create_node_route = create_node(Arc::clone(&nodes), difficulty, port);
    let get_chain_route = get_chain(Arc::clone(&nodes));
    let add_block_route = add_block(Arc::clone(&nodes));
    let sync_nodes_route = sync_nodes(Arc::clone(&nodes));

    let routes = create_node_route
        .or(get_chain_route)
        .or(add_block_route)
        .or(sync_nodes_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}
