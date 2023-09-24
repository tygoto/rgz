use anyhow::Result;
use tokio::signal;

use rgz::msgs::{StringMsg};
use rgz::transport::{Node};

async fn srv_echo(req: StringMsg) -> Result<StringMsg>{
    println!("ECHO: {}", req.data);
    Ok(req)
}

#[tokio::main]
async fn main() -> Result<()> {
    let topic = "/echo";
    let node = Node::new(None);
    node.advertise_service(topic, srv_echo, None).await?;
    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}