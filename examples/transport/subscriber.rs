use std::env;
use anyhow::Result;
use tokio::signal;

use rgz::msgs::StringMsg;
use rgz::transport::Node;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("GZ_IP", "172.17.0.1");

    let topic = "/foo";
    let mut node = Node::new(None);

    node.subscribe(topic, move |msg: StringMsg| {
        println!("RECV: {}", msg.data);
    })?;

    println!("Press Ctrl-C to exit.");
    signal::ctrl_c().await?;
    Ok(())
}
