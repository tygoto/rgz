use anyhow::Result;
use tokio::signal;

use rgz::msgs::StringMsg;
use rgz::transport::Node;

#[tokio::main]
async fn main() -> Result<()> {
    let topic = "/hello";
    let node = Node::new(None);
    node.advertise_service(topic, |req: StringMsg| {
        let res = StringMsg {
            data: format!("{}, World!", req.data),
            ..Default::default()
        };
        Ok(res)
    }, None)?;

    println!("Press Ctrl-C to exit.");
    signal::ctrl_c().await?;
    Ok(())
}
