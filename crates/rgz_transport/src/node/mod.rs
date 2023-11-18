mod node;
mod options;
mod shared;

pub use node::{Node, Publisher};
pub use options::{AdvertiseOptions, NodeOptions};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;

use crate::discovery::DiscoveryPublisher;
use crate::transport::{PublishMessage, ReplyMessage, RequestMessage};

const DEFAULT_DISCOVERY_IP: &str = "239.255.0.7";
const DEFAULT_MSG_DISC_PORT: u16 = 10317;
const DEFAULT_SRV_DISC_PORT: u16 = 10318;

#[derive(Debug, Clone)]
pub(crate) struct SubscribeArgs {
    n_uuid: String,
    topic: String,
    msg_type: String,
    sender: UnboundedSender<PublishMessage>,
}

pub(crate) enum NodeEvent {
    Advertise(DiscoveryPublisher),
    AdvertiseService(DiscoveryPublisher, UnboundedSender<RequestMessage>),
    Subscribe(SubscribeArgs),
    Publish(PublishMessage),
    Request(RequestMessage, oneshot::Sender<ReplyMessage>),
    Reply(ReplyMessage),
}

pub(crate) enum TransportEvent {
    Subscription(PublishMessage),
    Request(RequestMessage),
    Response(ReplyMessage),
}
