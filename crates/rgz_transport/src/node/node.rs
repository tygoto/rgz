use std::future::Future;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use anyhow::{bail, Result};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::time::{timeout_at, Instant};
use tracing::{error};

use crate::discovery::{
    DiscoveryMsgPublisher, DiscoveryPubType, DiscoveryPublisher, DiscoverySrvPublisher,
};
use crate::node::shared::NodeShared;
use crate::node::{AdvertiseOptions, NodeEvent, NodeOptions, SubscribeArgs};
use crate::transport::{PublishMessage, ReplyMessage, RequestMessage};
use crate::utils::topic as topic_utils;
use rgz_msgs::GzMessage;

pub struct Node {
    // Node UUID. This ID is unique for each node.
    n_uuid: String,

    // The shared data between all nodes.
    node_shared: Arc<Mutex<NodeShared>>,

    // Custom options for this node.
    node_options: NodeOptions,
}

impl Node {
    pub fn new(options: Option<NodeOptions>) -> Self {
        let node_options = options.unwrap_or_default();
        let n_uuid = uuid::Uuid::new_v4().to_string();
        let node_shared = NodeShared::instance();

        Node {
            n_uuid,
            node_shared,
            node_options,
        }
    }

    fn create_fully_qualified_topic(&self, topic: &str) -> Result<String> {
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();

        // Remap the topic
        if let Some(remap_topic) = self.node_options.topic_remap(&topic) {
            topic = remap_topic.clone();
        }
        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;
        Ok(fully_qualified_topic)
    }

    pub fn advertise<T>(
        &self,
        topic: &str,
        options: Option<AdvertiseOptions>,
    ) -> Result<Publisher<T>>
        where
            T: GzMessage,
    {
        let advertise_options = options.unwrap_or_default();
        let fully_qualified_topic = self.create_fully_qualified_topic(topic)?;

        // Create the discovery publisher
        let message_publisher = DiscoveryMsgPublisher {
            ctrl: "unused".to_string(),
            msg_type: T::TYPE_NAME.to_string(),
            throttled: advertise_options.throttled(),
            msgs_per_sec: advertise_options.msgs_per_sec(),
        };
        let pub_type = DiscoveryPubType::MsgPub(message_publisher);

        let discovery_publisher = DiscoveryPublisher {
            topic: fully_qualified_topic.clone(),
            address: "unset".to_string(),
            process_uuid: "unset".to_string(),
            node_uuid: self.n_uuid.to_string(),
            scope: advertise_options.scope() as i32,
            pub_type: Some(pub_type),
        };

        let event_sender = {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.advertise(discovery_publisher)?
        };

        Ok(Publisher::<T>::new(
            &fully_qualified_topic,
            advertise_options,
            event_sender,
        ))
    }

    pub fn subscribe<T, F>(&mut self, topic: &str, mut cb: F) -> Result<()>
        where
            T: GzMessage + Default,
            F: FnMut(T) + Send + 'static,
    {
        let fully_qualified_topic = self.create_fully_qualified_topic(topic)?;
        let (msg_sender, mut msg_receiver) =
            mpsc::unbounded_channel::<PublishMessage>();
        {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.subscribe(SubscribeArgs {
                n_uuid: self.n_uuid.to_string(),
                topic: fully_qualified_topic.clone(),
                msg_type: T::TYPE_NAME.to_string(),
                sender: msg_sender,
            })?;
        };

        tokio::spawn(async move {
            while let Some(mut msgs) = msg_receiver.recv().await {
                if let Ok(msg) = T::decode(&msgs.data[..]) {
                    cb(msg);
                } else {
                    error!("Failed to decode request");
                }
            }
        });
        Ok(())
    }

    pub fn advertise_service<REQ, RES, F>(
        &self, topic: &str, mut cb: F, options: Option<AdvertiseOptions>,
    ) -> Result<()>
        where
            REQ: GzMessage + Default,
            RES: GzMessage + Default,
            F: FnMut(REQ) -> Result<RES> + Send + 'static,
    {
        let advertise_options = options.unwrap_or_default();

        // Remap the topic
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();
        if let Some(remap_topic) = self.node_options.topic_remap(&topic) {
            topic = remap_topic.clone();
        }
        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;

        // Create the discovery publisher
        let service_publisher = DiscoverySrvPublisher {
            socket_id: "".to_string(),
            request_type: REQ::TYPE_NAME.to_string(),
            response_type: RES::TYPE_NAME.to_string(),
        };
        let pub_type = DiscoveryPubType::SrvPub(service_publisher);

        let discovery_publisher = DiscoveryPublisher {
            topic: fully_qualified_topic.to_string(),
            address: "unset".to_string(),
            process_uuid: "unset".to_string(),
            node_uuid: self.n_uuid.to_string(),
            scope: advertise_options.scope() as i32,
            pub_type: Some(pub_type),
        };

        let (request_sender, mut request_receiver) = mpsc::unbounded_channel::<RequestMessage>();

        let event_sender = {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.advertise_service(discovery_publisher, request_sender)?
        };

        tokio::spawn(async move {
            while let Some(mut msgs) = request_receiver.recv().await {
                if let Ok(msg) = REQ::decode(&msgs.data[..]) {
                    let mut data = vec![];
                    let mut result = false;

                    if let Ok(res) = cb(msg) {
                        data = res.encode_to_vec();
                        result = true;
                    } else {
                        error!("Failed to call service");
                    }

                    let address = msgs.requester_address;
                    if let Err(e) = event_sender.send(NodeEvent::Reply(ReplyMessage {
                        requester_address: Some(address),
                        requester_id: msgs.requester_id.to_string(),
                        topic: msgs.topic.to_string(),
                        node_uuid: msgs.node_uuid.to_string(),
                        req_uuid: msgs.req_uuid.to_string(),
                        data,
                        result,
                    }))
                    {
                        error!("Failed to send reply: {}", e);
                    }
                } else {
                    error!("Failed to decode request");
                }
            }
        });
        Ok(())
    }

    pub async fn request<REQ, RES>(
        &self,
        topic: &str,
        request: Option<REQ>,
        timeout: Option<Duration>,
    ) -> Result<Option<RES>>
        where
            REQ: GzMessage + Default,
            RES: GzMessage + Default,
    {
        let timeout = timeout.unwrap_or(Duration::from_millis(1000));

        // Remap the topic
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();
        if let Some(remap_topic) = self.node_options.topic_remap(&topic) {
            topic = remap_topic.clone();
        }
        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;

        let mut data = match request {
            Some(req) => req.encode_to_vec(),
            None => vec![],
        };

        let response_receiver = {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.request(RequestMessage {
                replier_address: None,
                replier_id: "unset".to_string(),
                topic: fully_qualified_topic.to_string(),
                requester_address: "unset".to_string(),
                requester_id: "unset".to_string(),
                node_uuid: self.n_uuid.to_string(),
                req_uuid: uuid::Uuid::new_v4().to_string(),
                data,
                req_type: REQ::TYPE_NAME.to_string(),
                res_type: RES::TYPE_NAME.to_string(),
            })?
        };

        let deadline = Instant::now() + timeout;
        match timeout_at(deadline, response_receiver).await {
            Ok(Ok(msg)) => {
                if msg.result {
                    let res = RES::decode(&msg.data[..])?;
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            }
            Err(_) => bail!("Did not receive value within {} ms", timeout.as_millis()),
            _ => bail!("Failed to receive value"),
        }
    }
}

#[derive(Clone)]
pub struct Publisher<T> {
    topic: String,
    options: AdvertiseOptions,
    sender: UnboundedSender<NodeEvent>,
    last_sent_msg: Option<Instant>,
    is_ready: Arc<AtomicBool>,
    _phantom: PhantomData<T>,
}

impl<T> Publisher<T>
    where
        T: GzMessage,
{
    fn new(topic: &str, options: AdvertiseOptions, sender: UnboundedSender<NodeEvent>) -> Self {
        let is_ready = Arc::new(AtomicBool::new(false));
        let is_ready_clone = is_ready.clone();
        tokio::spawn(async move {
            // Wait for the publisher to be registered
            tokio::time::sleep(Duration::from_millis(150)).await;
            is_ready_clone.store(true, Ordering::Relaxed);
        });

        Publisher {
            topic: topic.to_string(),
            options,
            sender,
            last_sent_msg: None,
            is_ready,
            _phantom: PhantomData,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::Relaxed)
    }

    pub fn publish(&self, msg: T) -> Result<()> {
        // TODO: Implement throttling

        if !self.is_ready.load(Ordering::Relaxed) {
            bail!("Publisher not ready");
        }

        self.sender.send(NodeEvent::Publish(PublishMessage {
            topic: self.topic.clone(),
            publisher_address: "unset".to_string(),
            msg_type: T::TYPE_NAME.to_string(),
            data: msg.encode_to_vec(),
        }))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use rgz_msgs::StringMsg;

    use futures::stream::StreamExt;
    use futures::channel::mpsc::channel as futures_channel;

    use super::*;

    #[tokio::test]
    async fn test_pub_sub() {
        let topic = "/foo";
        let mut node = Node::new(None);

        let recv_msg = Arc::new(Mutex::new(None));
        let m = recv_msg.clone();
        node.subscribe(topic, move |msg: StringMsg| {
            *m.lock().unwrap() = Some(msg);
        }).unwrap();

        let publisher = node.advertise::<StringMsg>(topic, None).unwrap();
        while !publisher.is_ready() {
            println!("Waiting for publisher to be ready...");
            sleep(Duration::from_millis(200)).await;
        }

        let str_msg = StringMsg {
            data: "hello world".to_string(),
            ..Default::default()
        };
        publisher.publish(str_msg).unwrap();

        sleep(Duration::from_millis(100)).await;

        {
            let msg = recv_msg.lock().unwrap();
            assert_eq!(msg.is_some(), true);
            let string_msg = msg.as_ref().unwrap();
            assert_eq!(string_msg.data, "hello world".to_string());
        }
    }

    #[tokio::test]
    async fn test_pub_sub_stream() {
        let topic = "/foo";
        let mut node = Node::new(None);

        let (mut sender, mut receiver) = futures_channel::<StringMsg>(10);

        node.subscribe(topic, move |msg: StringMsg| {
            if let Err(e) = sender.try_send(msg) {
                eprintln!("error: {}", e);
            }
        }).unwrap();

        let publisher = node.advertise::<StringMsg>(topic, None).unwrap();
        while !publisher.is_ready() {
            println!("Waiting for publisher to be ready...");
            sleep(Duration::from_millis(200)).await;
        }

        for i in 0..10 {
            let str_msg = StringMsg {
                data: format!("hello world: {}", i),
                ..Default::default()
            };
            publisher.publish(str_msg).unwrap();
            let msg = receiver.next().await.unwrap();
            assert_eq!(msg.data, format!("hello world: {}", i));
        }
    }

    #[tokio::test]
    async fn test_req_res() {
        let topic = "/echo";
        let node = Node::new(None);
        node.advertise_service(topic, move |req: StringMsg| {
            Ok(req)
        }, None).unwrap();

        let str_msg = StringMsg {
            data: "HELLO".to_string(),
            ..Default::default()
        };
        let request = Some(str_msg);
        let timeout = Some(Duration::from_secs(1));
        let res = node
            .request::<StringMsg, StringMsg>(topic, request, timeout)
            .await.unwrap();

        assert_eq!(res.is_some(), true);
        let string_msg = res.unwrap();
        assert_eq!(string_msg.data, "HELLO".to_string());
    }

    #[tokio::test]
    async fn test_req_res_error() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        let topic = "/echo";
        let node = Node::new(None);
        node.advertise_service(topic, move |req: StringMsg| {
            bail!("error");
            Ok(req)
        }, None).unwrap();

        let str_msg = StringMsg {
            data: "HELLO".to_string(),
            ..Default::default()
        };
        let request = Some(str_msg);
        let timeout = Some(Duration::from_secs(1));
        let res = node
            .request::<StringMsg, StringMsg>(topic, request, timeout)
            .await.unwrap();

        assert_eq!(res.is_some(), false);
    }

}
