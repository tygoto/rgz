#[path = "gz.msgs.rs"]
mod msgs;

pub use msgs::*;


pub trait GzMessage: prost::Message {
    const TYPE_NAME: &'static str;
}
