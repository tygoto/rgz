use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, process};

use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use tokio::select;

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

use tracing::{debug, error, info, trace};

use crate::discovery::{
    Discovery, DiscoveryMsgPublisher, DiscoveryPubType, DiscoveryPublisher, DiscoverySrvPublisher,
    DiscoveryStore,
};
use crate::dispatcher::{
    CleanFunction, DeleteFunction, Dispatcher, DispatcherStore, PendingRequest, ResponseDispatcher,
    ServiceDispatcher, Subscriber,
};
use crate::node::{
    NodeEvent, SubscribeArgs, TransportEvent, DEFAULT_DISCOVERY_IP, DEFAULT_MSG_DISC_PORT,
    DEFAULT_SRV_DISC_PORT,
};
use crate::transport::{PublishMessage, ReplyMessage, RequestMessage, Transporter};
use crate::utils::env as env_utils;

#[derive(Debug)]
enum DiscoveryEvent {
    Connection(DiscoveryPublisher),
    Disconnection(DiscoveryPublisher),
    Registration(DiscoveryPublisher),
    Unregistration(DiscoveryPublisher),
    SrvConnection(DiscoveryPublisher),
    SrvDisconnection(DiscoveryPublisher),
}

static NODE_SHARED_MAP: Lazy<Mutex<HashMap<u32, Arc<Mutex<NodeShared>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(crate) struct NodeShared {
    p_uuid: String,
    discovery_ip: String,
    msg_disc_port: u16,
    srv_disc_port: u16,
    node_event_sender: Option<UnboundedSender<NodeEvent>>,
    verbose: bool,
    handle: Option<tokio::task::JoinHandle<()>>,
}
impl NodeShared {
    pub fn instance() -> Arc<Mutex<NodeShared>> {
        let pid = process::id();
        let mut node_shared_map = NODE_SHARED_MAP.lock().unwrap();
        return if let Some(node_shared) = node_shared_map.get(&pid) {
            node_shared.clone()
        } else {
            // No instance found, create a new one and insert it into the map.
            let mut new_node_shared = NodeShared::new();
            new_node_shared.start();
            std::thread::sleep(std::time::Duration::from_millis(5));
            let new_node_shared = Arc::new(Mutex::new(new_node_shared));
            node_shared_map.insert(pid, new_node_shared.clone());
            new_node_shared
        };
    }
    fn new() -> Self {
        let mut verbose = false;
        let p_uuid = uuid::Uuid::new_v4().to_string();
        let mut discovery_ip = DEFAULT_DISCOVERY_IP.to_string();
        // If GZ_VERBOSE=1 enable the verbose mode.
        if let Ok(gz_verbose) = env::var("GZ_VERBOSE") {
            if !gz_verbose.is_empty() {
                verbose = gz_verbose == "1";
            }
        }
        // Set the multicast IP used for discovery.
        if let Ok(ip) = env::var("GZ_DISCOVERY_MULTICAST_IP") {
            if !ip.is_empty() {
                discovery_ip = ip;
            }
        }
        // Set the port used for msg discovery.
        let msg_disc_port =
            env_utils::non_negative_env_var("GZ_DISCOVERY_MSG_PORT", DEFAULT_MSG_DISC_PORT);
        // Set the port used for srv discovery.
        let mut srv_disc_port =
            env_utils::non_negative_env_var("GZ_DISCOVERY_SRV_PORT", DEFAULT_SRV_DISC_PORT);

        // Sanity check: the discovery ports should be unique.
        if msg_disc_port == srv_disc_port {
            if msg_disc_port < 65535 {
                srv_disc_port += 1;
            } else {
                srv_disc_port -= 1;
            }
            eprintln!(
                "Your discovery ports are the same [{}]. \
                Using [{}] for messages and [{}] for services",
                msg_disc_port, msg_disc_port, srv_disc_port
            );
        }

        NodeShared {
            p_uuid,
            discovery_ip,
            msg_disc_port,
            srv_disc_port,
            node_event_sender: None,
            verbose,
            handle: None,
        }
    }
    fn start(&mut self) {
        let mut inner = NodeSharedInner::new(
            &self.p_uuid,
            &self.discovery_ip,
            self.msg_disc_port,
            self.srv_disc_port,
            self.verbose,
        );
        let node_event_sender = inner.node_event_sender();
        self.node_event_sender = Some(node_event_sender);

        let handle = tokio::spawn(async move {
            inner.run().await;
        });
        self.handle = Some(handle);
    }
    pub(crate) fn advertise(
        &mut self,
        discovery_publisher: DiscoveryPublisher,
    ) -> Result<UnboundedSender<NodeEvent>> {
        match self.node_event_sender.as_ref() {
            None => bail!("Node is not started"),
            Some(node_event_sender) => {
                node_event_sender.send(NodeEvent::Advertise(discovery_publisher))?;
                Ok(node_event_sender.clone())
            }
        }
    }
    pub(crate) fn advertise_service(
        &mut self,
        discovery_publisher: DiscoveryPublisher,
        request_sender: UnboundedSender<RequestMessage>,
    ) -> Result<UnboundedSender<NodeEvent>> {
        match self.node_event_sender.as_ref() {
            None => bail!("Node is not started"),
            Some(node_event_sender) => {
                node_event_sender.send(NodeEvent::AdvertiseService(
                    discovery_publisher,
                    request_sender,
                ))?;
                Ok(node_event_sender.clone())
            }
        }
    }
    pub(crate) fn subscribe(&mut self, args: SubscribeArgs) -> Result<()> {
        match self.node_event_sender.as_ref() {
            None => bail!("Node is not started"),
            Some(node_event_sender) => {
                node_event_sender.send(NodeEvent::Subscribe(args))?;
                Ok(())
            }
        }
    }
    pub(crate) fn request(
        &mut self,
        msg: RequestMessage,
    ) -> Result<oneshot::Receiver<ReplyMessage>> {
        match self.node_event_sender.as_ref() {
            None => bail!("Node is not started"),
            Some(node_event_sender) => {
                let (sender, receiver) = oneshot::channel::<ReplyMessage>();
                node_event_sender.send(NodeEvent::Request(msg, sender))?;
                Ok(receiver)
            }
        }
    }
}

impl Drop for NodeShared {
    fn drop(&mut self) {
        let pid = process::id();
        let mut node_shared_map = NODE_SHARED_MAP.lock().unwrap();
        node_shared_map.remove(&pid);
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}

struct NodeSharedInner {
    p_uuid: String,

    discovery_event_receiver: UnboundedReceiver<DiscoveryEvent>,
    discovery_event_sender: UnboundedSender<DiscoveryEvent>,

    node_event_receiver: UnboundedReceiver<NodeEvent>,
    node_event_sender: UnboundedSender<NodeEvent>,
    transport_event_receiver: UnboundedReceiver<TransportEvent>,

    msg_discovery: Discovery,
    srv_discovery: Discovery,
    transporter: Transporter,

    subscribers: DispatcherStore<Subscriber>,
    pending_requests: DispatcherStore<PendingRequest>,
    response_dispatchers: DispatcherStore<ResponseDispatcher>,
    services: DispatcherStore<ServiceDispatcher>,

    verbose: bool,
}

impl NodeSharedInner {
    fn new(
        p_uuid: &str,
        discovery_ip: &str,
        msg_disc_port: u16,
        srv_disc_port: u16,
        verbose: bool,
    ) -> Self {
        let (discovery_event_sender, discovery_event_receiver) =
            mpsc::unbounded_channel::<DiscoveryEvent>();
        let (node_event_sender, node_event_receiver) = mpsc::unbounded_channel::<NodeEvent>();
        let (transport_event_sender, transport_event_receiver) =
            mpsc::unbounded_channel::<TransportEvent>();

        // msg discovery
        let mut msg_discovery = Discovery::new(p_uuid, discovery_ip, msg_disc_port, verbose);

        let sender = discovery_event_sender.clone();
        msg_discovery.set_connection_cb(move |discovery_publisher| {
            sender
                .send(DiscoveryEvent::Connection(discovery_publisher))
                .unwrap();
        });
        let sender = discovery_event_sender.clone();
        msg_discovery.set_disconnection_cb(move |discovery_publisher| {
            sender
                .send(DiscoveryEvent::Disconnection(discovery_publisher))
                .unwrap();
        });
        let sender = discovery_event_sender.clone();
        msg_discovery.set_registration_cb(move |discovery_publisher| {
            sender
                .send(DiscoveryEvent::Registration(discovery_publisher))
                .unwrap();
        });
        let sender = discovery_event_sender.clone();
        msg_discovery.set_unregistration_cb(move |discovery_publisher| {
            sender
                .send(DiscoveryEvent::Unregistration(discovery_publisher))
                .unwrap();
        });

        // srv discovery
        let mut srv_discovery = Discovery::new(p_uuid, discovery_ip, srv_disc_port, verbose);
        let sender = discovery_event_sender.clone();
        srv_discovery.set_connection_cb(move |discovery_publisher| {
            sender
                .send(DiscoveryEvent::SrvConnection(discovery_publisher))
                .unwrap();
        });
        let sender = discovery_event_sender.clone();
        srv_discovery.set_disconnection_cb(move |discovery_publisher| {
            sender
                .send(DiscoveryEvent::SrvDisconnection(discovery_publisher))
                .unwrap();
        });

        // transporter
        let host_addr = msg_discovery.host_addr().to_string();
        let mut transporter = Transporter::new(&host_addr);
        let sender = transport_event_sender.clone();
        transporter.set_subscription_handler(move |msg| {
            sender.send(TransportEvent::Subscription(msg)).unwrap();
        });
        let sender = transport_event_sender.clone();
        transporter.set_request_handler(move |msg| {
            sender.send(TransportEvent::Request(msg)).unwrap();
        });
        let sender = transport_event_sender.clone();
        transporter.set_response_handler(move |msg| {
            sender.send(TransportEvent::Response(msg)).unwrap();
        });

        NodeSharedInner {
            p_uuid: p_uuid.to_string(),
            discovery_event_receiver,
            discovery_event_sender,
            node_event_receiver,
            node_event_sender,
            transport_event_receiver,
            msg_discovery,
            srv_discovery,
            transporter,
            subscribers: DispatcherStore::new(),
            pending_requests: DispatcherStore::new(),
            response_dispatchers: DispatcherStore::new(),
            services: DispatcherStore::new(),
            verbose,
        }
    }

    fn node_event_sender(&self) -> UnboundedSender<NodeEvent> {
        self.node_event_sender.clone()
    }

    async fn run(&mut self) {
        self.msg_discovery.start();
        self.srv_discovery.start();
        self.transporter.start();

        loop {
            select! {
                Some(event) = self.discovery_event_receiver.recv() => {
                    let discovery_event: DiscoveryEvent = event;
                    match discovery_event {
                        DiscoveryEvent::Connection(discovery_publisher) => {
                            self.on_connection(discovery_publisher);
                        }
                        DiscoveryEvent::Disconnection(discovery_publisher) => {
                            self.on_disconnection(discovery_publisher);
                        }
                        DiscoveryEvent::Registration(discovery_publisher) => {
                            self.on_registration(discovery_publisher);
                        }
                        DiscoveryEvent::Unregistration(discovery_publisher) => {
                            self.on_unregistration(discovery_publisher);
                        }
                        DiscoveryEvent::SrvConnection(discovery_publisher) => {
                            self.on_srv_connection(discovery_publisher);
                        }
                        DiscoveryEvent::SrvDisconnection(discovery_publisher) => {
                            self.on_srv_disconnection(discovery_publisher);
                        }
                    }
                }

                Some(event) = self.node_event_receiver.recv() => {
                    let node_event: NodeEvent = event;
                    match node_event {
                        NodeEvent::Advertise(discovery_publisher) => {
                            self.on_advertise(discovery_publisher);
                        }
                        NodeEvent::AdvertiseService(discovery_publisher,
                            sender) => {
                            self.on_advertise_service(discovery_publisher, sender);
                        }
                        NodeEvent::Subscribe(args) => {
                            self.on_subscribe(args);
                        }
                        NodeEvent::Publish(msg) => {
                            self.on_publish(msg);
                        }
                        NodeEvent::Request(msg, sender) => {
                            self.on_request(msg, sender);
                        }
                        NodeEvent::Reply(msg) => {
                            self.on_reply(msg);
                        }
                    }
                }

                Some(event) = self.transport_event_receiver.recv() => {
                    let transport_event: TransportEvent = event;
                    match transport_event {
                        TransportEvent::Subscription(msg) => {
                            self.on_subscription(msg);
                        }
                        TransportEvent::Request(msg) => {
                            self.on_receive_request(msg);
                        }
                        TransportEvent::Response(msg) => {
                            self.on_response(msg);
                        }
                    }
                }
            }
        }
    }

    // Discovery Event Handler
    fn on_connection(&mut self, mut discovery_publisher: DiscoveryPublisher) {
        trace!("on_connection");

        let topic = discovery_publisher.topic.to_string();
        let address = discovery_publisher.address.to_string();
        let p_uuid = discovery_publisher.process_uuid.to_string();

        if self.verbose {
            println!("Connection callback");
            DiscoveryStore::print_publisher(&discovery_publisher);
        }

        // Check if we are interested in this topic.
        if !self.subscribers.has_topic(&topic) || p_uuid == self.p_uuid {
            return;
        }

        // Subscribe to the topic.
        self.transporter
            .subscribe(Some(&address), None, Some(&topic), None);

        // Create a new publisher.
        let message_publisher = match discovery_publisher.pub_type {
            Some(DiscoveryPubType::MsgPub(ref mut msg_pub)) => msg_pub,
            _ => return,
        };
        message_publisher.ctrl = discovery_publisher.process_uuid.to_string();
        discovery_publisher.process_uuid = self.p_uuid.clone();

        // Register the new publisher.
        if let Some(subscribers) = self.subscribers.get_for_topic(&topic) {
            for s in subscribers {
                let mut p = discovery_publisher.clone();
                p.node_uuid = s.node_uuid().to_string();
                if let Err(err) = self.msg_discovery.register(p) {
                    error!("Failed to register: {}", err);
                }
            }
        }
    }
    fn on_disconnection(&mut self, discovery_publisher: DiscoveryPublisher) {
        trace!("on_disconnection");
        let topic = discovery_publisher.topic.as_str();
        let process_uuid = discovery_publisher.process_uuid.as_str();
        let node_uuid = discovery_publisher.node_uuid.as_str();

        if self.verbose {
            println!("New disconnection detected");
            println!("\tProcess UUID: {}", process_uuid);
        }
        // A remote subscriber[s] has been disconnected.
        if topic != "" && node_uuid != "" {
            let _ = self.subscribers.remove_by_node(topic, node_uuid);
        } else {
            self.subscribers.del_by_process(process_uuid);
        }
    }
    fn on_registration(&mut self, mut discovery_publisher: DiscoveryPublisher) {
        trace!("on_registration");
        let message_publisher = match discovery_publisher.pub_type {
            Some(DiscoveryPubType::MsgPub(ref mut msg_pub)) => msg_pub,
            _ => return,
        };

        // Discard the message if the destination p_uuid is not me.
        if message_publisher.ctrl != self.p_uuid {
            return;
        }

        let process_uuid = discovery_publisher.process_uuid.as_str();
        let node_uuid = discovery_publisher.node_uuid.as_str();
        let topic = discovery_publisher.topic.as_str();
        let msg_type = message_publisher.msg_type.as_str();

        if self.verbose {
            println!("Registering a new remote connection");
            println!("\tProcess UUID: {}", process_uuid);
            println!("\tNode UUID: {}", node_uuid);
        }

        let subscriber = Subscriber::new(process_uuid, node_uuid, topic, msg_type, None);
        if let Err(err) = self.subscribers.register(subscriber) {
            error!("Failed to register dispatcher: {}", err);
        }
    }
    fn on_unregistration(&mut self, mut discovery_publisher: DiscoveryPublisher) {
        trace!("on_unregistration");
        let message_publisher = match discovery_publisher.pub_type {
            Some(DiscoveryPubType::MsgPub(ref mut msg_pub)) => msg_pub,
            _ => return,
        };

        // Discard the message if the destination p_uuid is not me.
        if message_publisher.ctrl != self.p_uuid {
            return;
        }
        let topic = discovery_publisher.topic.as_str();
        let process_uuid = discovery_publisher.process_uuid.as_str();
        let node_uuid = discovery_publisher.node_uuid.as_str();
        if self.verbose {
            println!("Registering the end of a remote connection");
            println!("\tProcess UUID: {}", process_uuid);
            println!("\tNode UUID: {}", node_uuid);
        }

        // Delete a remote subscriber.
        self.subscribers.remove_by_node(topic, node_uuid).unwrap();
    }

    fn on_srv_connection(&mut self, discovery_publisher: DiscoveryPublisher) {
        trace!("on_srv_connection");
        let service_publisher = match discovery_publisher.pub_type {
            Some(DiscoveryPubType::SrvPub(ref srv_pub)) => srv_pub,
            _ => return,
        };

        let topic = discovery_publisher.topic.to_string();
        let request_type = Some(service_publisher.request_type.as_str());
        let response_type = Some(service_publisher.response_type.as_str());
        let replier_address = discovery_publisher.address.as_str();
        let replier_id = service_publisher.socket_id.as_str();

        if self.verbose {
            info!("Service call connection callback");
            DiscoveryStore::print_publisher(&discovery_publisher);
        }

        self.send_pending_remote_reqs(
            &topic,
            request_type,
            response_type,
            replier_address,
            replier_id,
        );
    }
    fn on_srv_disconnection(&self, discovery_publisher: DiscoveryPublisher) {
        trace!("on_srv_disconnection");
        // Remove the address from the list of connected addresses.
        let address = discovery_publisher.address.as_str();
        self.transporter.srv_disconnect(address);

        if self.verbose {
            info!("Service call disconnection callback");
            DiscoveryStore::print_publisher(&discovery_publisher);
        }
    }

    // Node Event Handler
    fn on_advertise(&mut self, mut discovery_publisher: DiscoveryPublisher) {
        trace!("on_advertise");
        discovery_publisher.process_uuid = self.p_uuid.clone();
        discovery_publisher.address = self.transporter.publisher_address();
        if let Err(err) = self.msg_discovery.advertise(discovery_publisher) {
            error!("Failed to advertise: {}", err);
        }
    }
    fn on_advertise_service(
        &mut self,
        mut discovery_publisher: DiscoveryPublisher,
        request_sender: UnboundedSender<RequestMessage>,
    ) {
        trace!("on_advertise_service");
        let service_publisher = match discovery_publisher.pub_type {
            Some(DiscoveryPubType::SrvPub(ref mut srv_pub)) => srv_pub,
            _ => return,
        };

        service_publisher.socket_id = self.transporter.replier_id();
        let request_type = Some(service_publisher.request_type.as_str());
        let response_type = Some(service_publisher.response_type.as_str());

        discovery_publisher.process_uuid = self.p_uuid.clone();
        discovery_publisher.address = self.transporter.replier_address();

        let node_uuid = discovery_publisher.node_uuid.as_str();
        let topic = discovery_publisher.topic.as_str();

        let service = ServiceDispatcher::new(
            node_uuid,
            topic,
            request_type,
            response_type,
            request_sender,
        );

        if let Err(err) = self.services.register(service) {
            info!("Failed to register dispatcher: {}", err);
        }
        // Advertise the service.
        self.srv_discovery.advertise(discovery_publisher).unwrap();
    }
    fn on_subscribe(&mut self, args: SubscribeArgs) {
        trace!("on_subscribe");
        let process_uuid = self.p_uuid.as_str();
        let node_uuid = args.n_uuid.as_str();
        let topic = args.topic.as_str();
        let msg_type = args.msg_type.as_str();
        let sender = Some(args.sender.clone());
        let subscriber = Subscriber::new(process_uuid, node_uuid, topic, msg_type, sender);
        if let Err(err) = self.subscribers.register(subscriber) {
            error!("Failed to register subscriber: {}", err);
        }
        if let Err(err) = self.msg_discovery.discover(topic) {
            debug!("Failed to discover: {}", err);
        }
    }
    fn on_publish(&mut self, msg: PublishMessage) {
        trace!("on_publish {:?}", msg);

        if let Some(dispatchers) = self
            .subscribers
            .filter(&msg.topic, Some(&msg.msg_type), None)
        {
            for mut dispatcher in dispatchers {
                let publish_message = msg.clone();
                if dispatcher.is_remote() {
                    // Send the message to the remote subscriber.
                    if let Err(err) = self.transporter.publish(publish_message) {
                        error!("Failed to publish: {}", err);
                    }
                } else {
                    // Send the message to the local subscriber.
                    dispatcher
                        .dispatch(publish_message)
                        .expect("Failed to dispatch message");
                }
            }
        }
    }
    fn on_request(
        &mut self,
        mut request_message: RequestMessage,
        sender: oneshot::Sender<ReplyMessage>,
    ) {
        trace!("on_request {:?}", request_message);
        request_message.req_uuid = uuid::Uuid::new_v4().to_string();

        let req_msg = request_message.clone();
        let topic = req_msg.topic.as_str();
        let request_type = Some(req_msg.req_type.as_str());
        let response_type = Some(req_msg.res_type.as_str());

        // Request a local service.
        if let Some(service) = self.services.find(topic, request_type, response_type) {
            let dispatcher = ResponseDispatcher::new(&request_message, Some(sender));
            if let Err(err) = self.response_dispatchers.register(dispatcher) {
                error!("Failed to register dispatcher: {}", err);
            }
            service.request(request_message).expect("Failed to request");
            return;
        }

        // pending message
        let handler = PendingRequest::new(request_message, sender);
        if let Err(err) = self.pending_requests.register(handler) {
            error!("Failed to register dispatcher: {}", err);
        }

        if let Some(discovery_publishers) = self.srv_discovery.publishers(&topic) {
            for discovery_publisher in discovery_publishers {
                let service_publisher = match discovery_publisher.pub_type {
                    Some(DiscoveryPubType::SrvPub(ref srv_pub)) => srv_pub,
                    _ => return,
                };
                if Some(service_publisher.request_type.as_str()) == request_type
                    && Some(service_publisher.response_type.as_str()) == response_type
                {
                    let replier_address = discovery_publisher.address.as_str();
                    let replier_id = service_publisher.socket_id.as_str();
                    self.send_pending_remote_reqs(
                        &topic,
                        request_type,
                        response_type,
                        replier_address,
                        replier_id,
                    );
                    break;
                }
            }
        } else {
            // Discover the service.
            if let Err(err) = self.srv_discovery.discover(&topic) {
                debug!("Failed to discover: {}", err);
            }
        }
    }
    // Return service results.
    fn on_reply(&mut self, reply_message: ReplyMessage) {
        trace!("on_reply {:?}", reply_message);

        let topic = reply_message.topic.as_str();
        let node_uuid = reply_message.node_uuid.as_str();
        let handler_uuid = reply_message.req_uuid.as_str();

        if let Some(dispatcher) = self
            .response_dispatchers
            .get(topic, node_uuid, handler_uuid)
        {
            if dispatcher.is_remote() {
                // Send the reply to the remote requester.
                if let Err(err) = self.transporter.reply(reply_message) {
                    error!("Failed to reply: {}", err);
                }
            } else {
                // Send the reply to the local requester.
                if let Err(err) = dispatcher.dispatch(reply_message) {
                    error!("Failed to reply: {}", err);
                }
            }
            dispatcher.done();
        }
        self.response_dispatchers.clean(None);
    }

    // Transport Event Handler
    fn on_subscription(&mut self, msg: PublishMessage) {
        trace!("on_subscription {:?}", msg);
        if let Some(mut subscribers) =
            self.subscribers
                .filter(&msg.topic, Some(&msg.msg_type), None)
        {
            for subscriber in subscribers {
                if !subscriber.is_remote() {
                    subscriber
                        .dispatch(msg.clone())
                        .expect("Failed to dispatch message");
                }
            }
        }
    }
    fn on_receive_request(&mut self, request_message: RequestMessage) {
        trace!("on_receive_request {:?}", request_message);
        let topic = request_message.topic.as_str();
        let request_type = Some(request_message.req_type.as_str());
        let response_type = Some(request_message.res_type.as_str());

        // service
        if let Some(service) = self.services.find(topic, request_type, response_type) {
            let dispatcher = ResponseDispatcher::new(&request_message, None);
            if let Err(err) = self.response_dispatchers.register(dispatcher) {
                error!("Failed to register dispatcher: {}", err);
            }
            service.request(request_message).expect("Failed to request");
        } else {
            error!("Service not found");
        }
    }
    fn on_response(&mut self, msg: ReplyMessage) {
        trace!("on_response {:?}", msg);
        // Remove the handler associated to this service request.
        // We won't receive a response because this is a oneway request.

        if let Some(dispatcher) =
            self.response_dispatchers
                .get(&msg.topic, &msg.node_uuid, &msg.req_uuid)
        {
            dispatcher
                .dispatch(msg)
                .expect("Failed to dispatch message");
            dispatcher.done();
        } else {
            error!("ResponseDispatcher not found");
        }
        self.response_dispatchers.clean(None);
    }

    fn send_pending_remote_reqs(
        &mut self,
        topic: &str,
        request_type: Option<&str>,
        response_type: Option<&str>,
        replier_address: &str,
        replier_id: &str,
    ) {
        if self.verbose {
            info!("Found a service call at [{}]", replier_address);
        }

        if let Some(requests) = self
            .pending_requests
            .filter(topic, request_type, response_type)
        {
            for request in requests {
                let mut request_message = match request.message() {
                    Some(request_message) => request_message,
                    None => continue,
                };
                let sender = match request.sender() {
                    Some(sender) => Some(sender),
                    None => continue,
                };
                request_message.replier_address = Some(replier_address.to_string());
                request_message.replier_id = replier_id.to_string();

                let dispatcher = ResponseDispatcher::new(&request_message, sender);
                if let Err(err) = self.response_dispatchers.register(dispatcher) {
                    error!("Failed to register dispatcher: {}", err);
                }
                if let Err(err) = self.transporter.request(request_message) {
                    error!("Failed to request: {}", err);
                }
            }
        }
        self.pending_requests.clean(None);
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;
    use std::time::Duration;
    use tokio::time;
    // use tracing::subscriber;
    // use tracing_subscriber::FmtSubscriber;
    use crate::discovery::{
        DiscoveryMsgPublisher, DiscoveryPubType, DiscoveryScope, DiscoverySrvPublisher,
    };

    use super::*;

    const MSG_PORT: u16 = 11319;
    const SRV_PORT: u16 = 11320;
    const IP: &str = "224.0.0.7";

    const TOPIC: &str = "test_topic";
    const SERVICE_NAME: &str = "test_service";
    const ADDR1: &str = "tcp://127.0.0.1:12345";
    const CTRL1: &str = "tcp://127.0.0.1:12346";
    const P_UUID1: &str = "00000000-0000-0000-0000-000000000011";
    const N_UUID1: &str = "00000000-0000-0000-0000-000000000012";
    const ADDR2: &str = "tcp://127.0.0.1:12347";
    const CTRL2: &str = "tcp://127.0.0.1:12348";
    const P_UUID2: &str = "00000000-0000-0000-0000-000000000021";
    const N_UUID2: &str = "00000000-0000-0000-0000-000000000022";

    #[derive(Clone, PartialEq, ::prost::Message)]
    struct Person {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub id: i32,
    }

    // Check that a node can pub & sub to the same process.
    #[tokio::test]
    async fn test_pub_sub_same_process() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let mut node_shared = NodeShared::new();
        node_shared.p_uuid = P_UUID1.to_string();
        node_shared.start();

        let msg_type = "Person";
        let message_publisher = DiscoveryMsgPublisher {
            ctrl: "unused".to_string(),
            msg_type: msg_type.to_string(),
            throttled: false,
            msgs_per_sec: u64::MAX,
        };
        let pub_type = DiscoveryPubType::MsgPub(message_publisher);
        let sender = node_shared
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: "".to_string(),
                process_uuid: "".to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: Some(pub_type),
            })
            .unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<PublishMessage>();

        node_shared
            .subscribe(SubscribeArgs {
                n_uuid: N_UUID2.to_string(),
                topic: TOPIC.to_string(),
                msg_type: msg_type.to_string(),
                sender: tx.clone(),
            })
            .unwrap();

        let recv_msg = Arc::new(Mutex::new(None));
        let m = recv_msg.clone();

        tokio::spawn(async move {
            let msg = rx.recv().await.unwrap();
            let person = Person::decode(&msg.data[..]).unwrap();
            *m.lock().unwrap() = Some(person);
        });

        let data = Person {
            name: "Alice".to_string(),
            id: 1234,
        }
        .encode_to_vec();

        sender
            .send(NodeEvent::Publish(PublishMessage {
                topic: TOPIC.to_string(),
                msg_type: msg_type.to_string(),
                data,
                publisher_address: "unset".to_string(),
            }))
            .unwrap();

        time::sleep(Duration::from_millis(300)).await;

        {
            let person = recv_msg.lock().unwrap();
            assert_eq!(person.is_some(), true);
            let person = person.as_ref().unwrap();
            assert_eq!(person.id, 1234);
            assert_eq!(person.name, "Alice");
        }
    }

    // Check that a node can pub & sub to another process.
    #[tokio::test]
    async fn test_pub_sub_another_process() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        let msg_type = "Person";

        let mut node_shared1 = NodeShared::new();
        node_shared1.p_uuid = P_UUID1.to_string();
        node_shared1.discovery_ip = IP.to_string();
        node_shared1.msg_disc_port = MSG_PORT;
        node_shared1.verbose = true;
        node_shared1.start();
        let message_publisher = DiscoveryMsgPublisher {
            ctrl: "unused".to_string(),
            msg_type: msg_type.to_string(),
            throttled: false,
            msgs_per_sec: u64::MAX,
        };
        let pub_type = DiscoveryPubType::MsgPub(message_publisher);
        let sender = node_shared1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: "".to_string(),
                process_uuid: "".to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: Some(pub_type),
            })
            .unwrap();

        let mut node_shared2 = NodeShared::new();
        node_shared2.p_uuid = P_UUID2.to_string();
        node_shared2.discovery_ip = IP.to_string();
        node_shared2.msg_disc_port = MSG_PORT;
        node_shared2.verbose = true;
        node_shared2.start();

        let (tx, mut rx) = mpsc::unbounded_channel::<PublishMessage>();
        node_shared2
            .subscribe(SubscribeArgs {
                n_uuid: "node_uuid".to_string(),
                topic: TOPIC.to_string(),
                msg_type: msg_type.to_string(),
                sender: tx.clone(),
            })
            .unwrap();

        tokio::spawn(async move {
            time::sleep(Duration::from_millis(300)).await;
            let data = Person {
                name: "Alice".to_string(),
                id: 1234,
            }
            .encode_to_vec();

            sender
                .send(NodeEvent::Publish(PublishMessage {
                    topic: TOPIC.to_string(),
                    msg_type: msg_type.to_string(),
                    data,
                    publisher_address: "unset".to_string(),
                }))
                .unwrap();
        });

        let msg = rx.recv().await.unwrap();
        let person = Person::decode(&msg.data[..]).unwrap();
        assert_eq!(person.name, "Alice");
        assert_eq!(person.id, 1234);

        drop(node_shared1);
        time::sleep(Duration::from_millis(1000)).await;
    }
    #[tokio::test]
    async fn test_req_res_same_process() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        let mut node_shared = NodeShared::new();
        node_shared.p_uuid = P_UUID1.to_string();
        node_shared.start();

        let msg_type = "Person";
        let service_publisher = DiscoverySrvPublisher {
            socket_id: "".to_string(),
            request_type: "Person".to_string(),
            response_type: "Person".to_string(),
        };
        let pub_type = DiscoveryPubType::SrvPub(service_publisher);
        let discovery_publisher = DiscoveryPublisher {
            topic: TOPIC.to_string(),
            address: "".to_string(),
            process_uuid: "".to_string(),
            node_uuid: N_UUID1.to_string(),
            scope: DiscoveryScope::All as i32,
            pub_type: Some(pub_type),
        };

        let (tx, mut rx) = mpsc::unbounded_channel::<RequestMessage>();
        let sender = node_shared
            .advertise_service(discovery_publisher, tx)
            .unwrap();

        tokio::spawn(async move {
            if let Some(msg) = rx.recv().await {
                let mut person = Person::decode(&msg.data[..]).unwrap();
                person.name = "Bob".to_string();
                let reply = ReplyMessage {
                    requester_address: Some(msg.requester_address),
                    requester_id: msg.requester_id,
                    topic: msg.topic,
                    node_uuid: msg.node_uuid,
                    req_uuid: msg.req_uuid,
                    data: person.encode_to_vec(),
                    result: true,
                };
                sender.send(NodeEvent::Reply(reply)).unwrap();
            }
        });

        time::sleep(Duration::from_millis(100)).await;

        let data = Person {
            name: "Alice".to_string(),
            id: 1234,
        }
        .encode_to_vec();

        let receiver = node_shared
            .request(RequestMessage {
                replier_address: None,
                replier_id: "unset".to_string(),
                topic: TOPIC.to_string(),
                requester_address: "unset".to_string(),
                requester_id: "unset".to_string(),
                node_uuid: N_UUID1.to_string(),
                req_uuid: "unset".to_string(),
                data,
                req_type: "Person".to_string(),
                res_type: "Person".to_string(),
            })
            .unwrap();

        // tokio::time::sleep(Duration::from_millis(300)).await;

        let msg = receiver.await.unwrap();
        let person = Person::decode(&msg.data[..]).unwrap();
        assert_eq!(person.id, 1234);
        assert_eq!(person.name, "Bob");
    }

    #[tokio::test]
    async fn test_req_res_another_process() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        let mut node_shared1 = NodeShared::new();
        node_shared1.p_uuid = P_UUID1.to_string();
        node_shared1.discovery_ip = IP.to_string();
        node_shared1.srv_disc_port = SRV_PORT;
        node_shared1.verbose = true;
        node_shared1.start();
        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut node_shared2 = NodeShared::new();
        node_shared2.p_uuid = P_UUID2.to_string();
        node_shared2.discovery_ip = IP.to_string();
        node_shared2.srv_disc_port = SRV_PORT;
        node_shared2.verbose = true;
        node_shared2.start();

        let msg_type = "Person";
        let service_publisher = DiscoverySrvPublisher {
            socket_id: "".to_string(),
            request_type: "Person".to_string(),
            response_type: "Person".to_string(),
        };
        let pub_type = DiscoveryPubType::SrvPub(service_publisher);
        let discovery_publisher = DiscoveryPublisher {
            topic: TOPIC.to_string(),
            address: "".to_string(),
            process_uuid: "".to_string(),
            node_uuid: N_UUID1.to_string(),
            scope: DiscoveryScope::All as i32,
            pub_type: Some(pub_type),
        };

        let (tx, mut rx) = mpsc::unbounded_channel::<RequestMessage>();
        let sender = node_shared1
            .advertise_service(discovery_publisher, tx)
            .unwrap();

        tokio::spawn(async move {
            if let Some(msg) = rx.recv().await {
                let mut person = Person::decode(&msg.data[..]).unwrap();
                person.name = "Bob".to_string();
                let reply = ReplyMessage {
                    requester_address: Some(msg.requester_address),
                    requester_id: msg.requester_id,
                    topic: msg.topic,
                    node_uuid: msg.node_uuid,
                    req_uuid: msg.req_uuid,
                    data: person.encode_to_vec(),
                    result: true,
                };
                sender.send(NodeEvent::Reply(reply)).unwrap();
            }
        });

        time::sleep(Duration::from_millis(100)).await;

        let data = Person {
            name: "Alice".to_string(),
            id: 1234,
        }
        .encode_to_vec();

        let receiver = node_shared2
            .request(RequestMessage {
                replier_address: None,
                replier_id: "unset".to_string(),
                topic: TOPIC.to_string(),
                requester_address: "unset".to_string(),
                requester_id: "unset".to_string(),
                node_uuid: N_UUID2.to_string(),
                req_uuid: "unset".to_string(),
                data,
                req_type: "Person".to_string(),
                res_type: "Person".to_string(),
            })
            .unwrap();

        let msg = receiver.await.unwrap();
        let person = Person::decode(&msg.data[..]).unwrap();
        assert_eq!(person.id, 1234);
        assert_eq!(person.name, "Bob");
    }
}
