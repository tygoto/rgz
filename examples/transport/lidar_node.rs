///
/// This is a Rust rewrite of lidar_node.c from the following tutorial.
/// https://gazebosim.org/docs/harmonic/sensors
///
use anyhow::Result;
use rgz::msgs::{LaserScan, Twist, Vector3d};
use rgz::transport::Node;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let topic_pub = "/cmd_vel";
    let mut node = Node::new(None);
    let publisher = node.advertise::<Twist>(topic_pub, None).await?;

    let topic_sub = "/lidar";
    node.subscribe(topic_sub, move |msg: LaserScan| {
        let p = publisher.clone();
        async move {
            let all_more = msg.ranges.iter().all(|&range| range >= 1.0);
            let mut twist = Twist {
                ..Default::default()
            };
            if all_more {
                twist.linear = Some(Vector3d {
                    x: 0.5,
                    ..Default::default()
                });
                twist.angular = Some(Vector3d {
                    z: 0.0,
                    ..Default::default()
                });
            } else {
                twist.linear = Some(Vector3d {
                    x: 0.0,
                    ..Default::default()
                });
                twist.angular = Some(Vector3d {
                    z: 0.5,
                    ..Default::default()
                });
            };
            p.publish(twist)?;

            Ok(())
        }
    })
    .await?;
    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}
