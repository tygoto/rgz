use anyhow::Result;
use tokio::signal;

use rgz::transport::node::{Node};
use rgz::msgs::StringMsg;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let topic = "/foo";
    let mut node = Node::new(None);

    node.subscribe(topic, |msg: StringMsg| async move {
        println!("RECV: {}", msg.data);
        Ok(())
    }).await?;

    signal::ctrl_c().await?;
    println!("ctrl-c received!");

    Ok(())
}