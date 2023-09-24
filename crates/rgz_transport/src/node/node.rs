use std::future::Future;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{bail, Result};
use tokio::time::{Instant, timeout_at};
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{error, info};

use rgz_msgs::GzMessage;
use crate::node::{AdvertiseOptions, NodeEvent, NodeOptions, SubscribeArgs};
use crate::node::shared::NodeShared;
use crate::discovery::{DiscoveryMsgPublisher, DiscoveryPublisher, DiscoveryPubType, DiscoverySrvPublisher};
use crate::transport::{PublishMessage, RequestMessage, ReplyMessage};
use crate::utils::topic as topic_utils;

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

    pub async fn advertise<T>(&self, topic: &str, options: Option<AdvertiseOptions>) -> Result<Publisher<T>>
        where T: GzMessage
    {
        let advertise_options = options.unwrap_or_default();

        // Remap the topic
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();
        if let Some(remap_topic) = self.node_options.topic_remap(&topic){
            topic = remap_topic.clone();
        }

        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;

        // Create the discovery publisher
        let message_publisher = DiscoveryMsgPublisher {
            ctrl: "unused".to_string(),
            msg_type: T::TYPE_NAME.to_string(),
            throttled: advertise_options.throttled(),
            msgs_per_sec: advertise_options.msgs_per_sec(),
        };
        let pub_type = DiscoveryPubType::MsgPub(message_publisher);

        let publisher = DiscoveryPublisher {
            topic: fully_qualified_topic.to_string(),
            address: "unset".to_string(),
            process_uuid: "unset".to_string(),
            node_uuid: self.n_uuid.to_string(),
            scope: advertise_options.scope() as i32,
            pub_type: Some(pub_type)
        };

        let event_sender = {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.advertise(publisher)?
        };

        // Wait for the publisher to be registered
        tokio::time::sleep(Duration::from_millis(200)).await;

        Ok(Publisher::<T>::new(&fully_qualified_topic, advertise_options, event_sender))
    }
    pub async fn subscribe<T, F, Fut>(&mut self, topic: &str, cb: F) -> Result<()>
        where
            T: GzMessage + Default + 'static,
            F: Fn(T) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send
    {
        // Remap the topic
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();
        if let Some(remap_topic) = self.node_options.topic_remap(&topic){
            topic = remap_topic.clone();
        }
        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;

        let (msg_sender, mut msg_receiver) =
            mpsc::unbounded_channel::<PublishMessage>();

        {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.subscribe(SubscribeArgs{
                n_uuid: self.n_uuid.to_string(),
                topic: fully_qualified_topic.to_string(),
                msg_type: T::TYPE_NAME.to_string(),
                sender: msg_sender,
            })?;
        };

        tokio::spawn(async move {
            while let Some(mut msgs) = msg_receiver.recv().await {

                if let Ok(msg) = T::decode(&msgs.data[..]) {
                    if let Err(err) = cb(msg).await {
                        error!("Failed to execute callback function: {}", err);
                    }
                }else{
                    error!("Failed to decode request");
                }
            }
        });
        Ok(())
    }

    pub async fn subscribe_raw<F, Fut>(&mut self, topic: &str, msg_type: &str, cb: F) -> Result<()>
        where
            F: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send
    {
        // Remap the topic
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();
        if let Some(remap_topic) = self.node_options.topic_remap(&topic){
            topic = remap_topic.clone();
        }
        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;

        let (msg_sender, mut msg_receiver) =
            mpsc::unbounded_channel::<PublishMessage>();

        {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.subscribe(SubscribeArgs{
                n_uuid: self.n_uuid.to_string(),
                topic: fully_qualified_topic.to_string(),
                msg_type: msg_type.to_string(),
                sender: msg_sender,
            })?;
        };

        tokio::spawn(async move {
            while let Some(mut msg) = msg_receiver.recv().await {
                if let Err(err) = cb(msg.data).await {
                    error!("Failed to execute callback function: {}", err);
                }
            }
        });
        Ok(())
    }

    pub async fn advertise_service<REQ, RES, F, Fut>(&self, topic: &str, cb: F,
                                                     options: Option<AdvertiseOptions>) -> Result<()>
        where REQ: GzMessage + Default,
              RES: GzMessage + Default,
              F: Fn(REQ) -> Fut + Send + Sync + 'static,
              Fut: Future<Output = Result<RES>> + Send + Sync + 'static
    {
        let advertise_options = options.unwrap_or_default();

        // Remap the topic
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();
        if let Some(remap_topic) = self.node_options.topic_remap(&topic){
            topic = remap_topic.clone();
        }
        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;

        // Create the discovery publisher
        let service_publisher = DiscoverySrvPublisher {
            socket_id: "".to_string(),
            request_type:  REQ::TYPE_NAME.to_string(),
            response_type: RES::TYPE_NAME.to_string(),
        };
        let pub_type = DiscoveryPubType::SrvPub(service_publisher);

        let discovery_publisher = DiscoveryPublisher {
            topic: fully_qualified_topic.to_string(),
            address: "unset".to_string(),
            process_uuid: "unset".to_string(),
            node_uuid: self.n_uuid.to_string(),
            scope: advertise_options.scope() as i32,
            pub_type: Some(pub_type)
        };

        let (request_sender, mut request_receiver) =
            mpsc::unbounded_channel::<RequestMessage>();

        let event_sender = {
            let mut node_shared = self.node_shared.lock().unwrap();
            node_shared.advertise_service(discovery_publisher, request_sender)?
        };

        tokio::spawn(async move {
            while let Some(mut msgs) = request_receiver.recv().await {
                if let Ok(msg) = REQ::decode(&msgs.data[..]) {
                    if let Ok(res) = cb(msg).await {
                        let address = msgs.requester_address;
                        event_sender.send(NodeEvent::Reply(ReplyMessage{
                            requester_address: Some(address),
                            requester_id: msgs.requester_id.to_string(),
                            topic: msgs.topic.to_string(),
                            node_uuid: msgs.node_uuid.to_string(),
                            req_uuid: msgs.req_uuid.to_string(),
                            data: res.encode_to_vec(),
                            result: false,
                        })).unwrap();
                    } else{
                        eprintln!("Failed to process request");
                    }
                }else{
                    eprintln!("Failed to decode request");
                }
            }
        });
        Ok(())
    }

    pub async fn request<REQ, RES>(&self, topic: &str, request: Option<REQ>, timeout: Option<Duration>)
        -> Result<RES>
        where REQ: GzMessage + Default,
              RES: GzMessage + Default
    {
        let timeout = timeout.unwrap_or(Duration::from_millis(1000));

        // Remap the topic
        let mut topic = topic.to_string();
        let partition = self.node_options.partition();
        let ns = self.node_options.namespace();
        if let Some(remap_topic) = self.node_options.topic_remap(&topic){
            topic = remap_topic.clone();
        }
        let fully_qualified_topic = topic_utils::fully_qualified_name(partition, ns, &topic)?;

        let mut data= match request {
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
                let res = RES::decode(&msg.data[..])?;
                Ok(res)
            }
            Err(_) => bail!("Did not receive value within {} ms", timeout.as_millis()),
            _ => bail!("Failed to receive value")
        }
    }

}

#[derive(Clone)]
pub struct Publisher <T>{
    topic: String,
    options: AdvertiseOptions,
    sender: UnboundedSender<NodeEvent>,
    last_sent_msg: Option<Instant>,
    _phantom: PhantomData<T>,
}

impl <T> Publisher <T> where T: GzMessage
{
    fn new(topic: &str, options: AdvertiseOptions, sender: UnboundedSender<NodeEvent>) -> Self {
        Publisher {
            topic: topic.to_string(),
            options,
            sender,
            last_sent_msg: None,
            _phantom: PhantomData,
        }
    }

    pub fn publish(&self, msg: T) -> Result<()> {
        // TODO: Implement throttling

        self.sender.send(NodeEvent::Publish(PublishMessage {
            topic: self.topic.clone(),
            publisher_address: "unset".to_string(),
            msg_type: T::TYPE_NAME.to_string(),
            data: msg.encode_to_vec(),
        }))?;
        
        Ok(())
    }
}
