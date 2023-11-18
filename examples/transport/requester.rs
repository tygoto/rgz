use anyhow::Result;
use rgz::msgs::StringMsg;
use rgz::transport::Node;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let topic = "/hello";
    let node = Node::new(None);

    let str_msg = StringMsg {
        data: "Hello".to_string(),
        ..Default::default()
    };
    let request = Some(str_msg);
    let timeout = Some(Duration::from_millis(3000));
    let res = node
        .request::<StringMsg, StringMsg>(topic, request, timeout).await?;
    println!("RES: {:?}", res);
    Ok(())
}
