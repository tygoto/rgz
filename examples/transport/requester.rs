use std::env;
use anyhow::Result;
use std::time::Duration;
use rgz::msgs::{StringMsg};
use rgz::transport::node::{Node};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let topic = "/echo";
    // env::set_var("GZ_VERBOSE", "1");

    let node = Node::new(None);

    let str_msg = StringMsg {
        data: "HELLO".to_string(),
        ..Default::default()
    };
    let request = Some(str_msg);
    let timeout = Some(Duration::from_secs(1));
    let res = node.request::<StringMsg,StringMsg>(topic, request, timeout).await?;
    println!("RES: {:?}", res);
    Ok(())
}