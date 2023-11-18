use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;

use rgz::msgs::StringMsg;
use rgz::transport::Node;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let topic = "/foo";
    let node = Node::new(None);
    let publisher = node.advertise(topic, None)?;

    while !publisher.is_ready() {
        println!("Waiting for publisher to be ready...");
        sleep(Duration::from_millis(200)).await;
    }

    for i in 0..100 {
        let str_msg = StringMsg {
            data: format!("hello world: {}", i),
            ..Default::default()
        };
        println!("Published message: {}", &str_msg.data);
        publisher.publish(str_msg)?;

        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
