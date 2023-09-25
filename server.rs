use block::error::BlockchainError;
use block::network::node::Node;
use block::storage::SledDb;
use std::env::{current_dir, self};
use std::sync::Arc;


#[tokio::main]
#[allow(unused_must_use)]
async fn main() -> Result<(),BlockchainError> {
    tracing_subscriber::fmt::init();

    let mut path = String::from("data2");
    if let Some(args) = env::args().nth(2) {
        path = args;
    }

    let path = current_dir().unwrap().join(path);
    let db = Arc::new(SledDb::new(path));
    let mut node = Node::new(db).await;
    node.start().await.unwrap();
    Ok(())
}
