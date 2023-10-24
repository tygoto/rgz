use anyhow::Result;
use std::env;
use tokio::signal;

use rgz::msgs::StringMsg;
use rgz::transport::Node;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // env::set_var("GZ_IP", "172.17.0.1");

    let topic = "/foo";
    let mut node = Node::new(None);

    node.subscribe(topic, |msg: StringMsg| async move {
        println!("RECV: {}", msg.data);
        Ok(())
    })?;

    signal::ctrl_c().await?;
    println!("ctrl-c received!");

    Ok(())
}
