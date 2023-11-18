use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration};

use anyhow::{bail, Result};
use tracing::{debug, error};
use zmq;

/// Timeout used for receiving messages (ms.).
const TIMEOUT: i64 = 250;

/// The high water mark of the receive message buffer.
const DEFAULT_RCV_HWM: i32 = 1000;

/// The high water mark of the send message buffer.
const DEFAULT_SND_HWM: i32 = 1000;

#[derive(Debug, Clone)]
pub(crate) struct PublishMessage {
    pub topic: String,
    pub publisher_address: String,
    pub msg_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct RequestMessage {
    pub replier_address: Option<String>,
    pub replier_id: String,
    pub topic: String,
    pub requester_address: String,
    pub requester_id: String,
    pub node_uuid: String,
    pub req_uuid: String,
    pub data: Vec<u8>,
    pub req_type: String,
    pub res_type: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ReplyMessage {
    pub requester_address: Option<String>,
    pub requester_id: String,
    pub topic: String,
    pub node_uuid: String,
    pub req_uuid: String,
    pub data: Vec<u8>,
    pub result: bool, // True when the service request was successful or false otherwise.
}

struct SubscribeEvent {
    connect: Option<String>,
    disconnect: Option<String>,
    subscribe: Option<String>,
    unsubscribe: Option<String>,
}

type SubscriptionHandlerType = Box<dyn Fn(PublishMessage) + Send>;
type RequestHandlerType = Box<dyn Fn(RequestMessage) + Send>;
type ResponseHandlerType = Box<dyn Fn(ReplyMessage) + Send>;
pub(crate) struct Transporter {
    host_addr: String,
    context: zmq::Context,
    publisher: zmq::Socket,
    requester: zmq::Socket,

    publisher_address: String,

    requester_id: String,
    requester_address: Arc<Mutex<String>>,

    replier_id: String,
    replier_address: Arc<Mutex<String>>,

    connections: Arc<Mutex<HashSet<String>>>,
    srv_connections: Arc<Mutex<HashSet<String>>>,

    subscription_handler: Arc<Mutex<Option<SubscriptionHandlerType>>>,
    request_handler: Arc<Mutex<Option<RequestHandlerType>>>,
    response_handler: Arc<Mutex<Option<ResponseHandlerType>>>,

    subscribe_evt_sender: Option<Sender<SubscribeEvent>>,
    reply_msg_sender: Option<Sender<ReplyMessage>>,
}

impl Transporter {
    pub(crate) fn new(host_addr: &str) -> Self {
        let any_tcp = format!("tcp://{}:*", host_addr);
        let context = zmq::Context::new();
        let publisher = context.socket(zmq::PUB).unwrap();
        let linger_val = 0;
        publisher.set_linger(linger_val).unwrap();
        publisher.set_sndhwm(DEFAULT_SND_HWM).unwrap();
        publisher.bind(&any_tcp).expect("failed binding publisher");

        let publisher_address = match publisher.get_last_endpoint() {
            Ok(Ok(endpoint)) => endpoint,
            _ => "".to_string(),
        };
        let requester = context.socket(zmq::ROUTER).unwrap();
        requester.set_linger(linger_val).unwrap();
        requester.set_router_mandatory(true).unwrap();
        let requester_id = uuid::Uuid::new_v4().to_string();
        let requester_address = Arc::new(Mutex::new("unset".to_string()));
        let replier_id = uuid::Uuid::new_v4().to_string();
        let replier_address = Arc::new(Mutex::new("unset".to_string()));

        Transporter {
            host_addr: host_addr.to_string(),
            context,
            publisher,
            requester,
            publisher_address,
            requester_id,
            requester_address,
            replier_id,
            replier_address,
            connections: Arc::new(Mutex::new(HashSet::new())),
            srv_connections: Arc::new(Mutex::new(HashSet::new())),
            subscription_handler: Arc::new(Mutex::new(None)),
            request_handler: Arc::new(Mutex::new(None)),
            response_handler: Arc::new(Mutex::new(None)),
            subscribe_evt_sender: None,
            reply_msg_sender: None,
        }
    }

    pub(crate) fn publisher_address(&self) -> String {
        self.publisher_address.clone()
    }

    pub(crate) fn replier_address(&self) -> String {
        self.replier_address.lock().unwrap().clone()
    }
    pub(crate) fn replier_id(&self) -> String {
        self.replier_id.clone()
    }

    pub(crate) fn subscribe(
        &self,
        connect: Option<&str>,
        disconnect: Option<&str>,
        subscribe_topic: Option<&str>,
        unsubscribe_topic: Option<&str>,
    ) {
        if let Some(event_sender) = self.subscribe_evt_sender.as_ref() {
            event_sender
                .send(SubscribeEvent {
                    connect: connect.map(|s| s.to_string()),
                    disconnect: disconnect.map(|s| s.to_string()),
                    subscribe: subscribe_topic.map(|s| s.to_string()),
                    unsubscribe: unsubscribe_topic.map(|s| s.to_string()),
                })
                .expect("subscribe failed");
        } else {
            error!("There is no sender. Transporter may not have started.");
        }
    }

    pub(crate) fn connections(&self) -> Vec<String> {
        let connections = self.connections.lock().unwrap();
        connections.iter().cloned().collect()
    }
    pub(crate) fn srv_disconnect(&self, address: &str) {
        self.srv_connections.lock().unwrap().remove(address);
    }

    pub(crate) fn publish(&self, publish_message: PublishMessage) -> Result<()> {
        let messages = {
            vec![
                zmq::Message::from(&publish_message.topic),
                zmq::Message::from(&self.publisher_address),
                zmq::Message::from(&publish_message.data),
                zmq::Message::from(&publish_message.msg_type),
            ]
        };
        let mut v = VecDeque::from(messages);
        while let Some(zmq_msg) = v.pop_front() {
            let flag = if v.is_empty() { 0 } else { zmq::SNDMORE };
            self.publisher.send(zmq_msg, flag)?;
        }

        Ok(())
    }

    pub(crate) fn request(&mut self, msg: RequestMessage) -> Result<()> {
        let address = match msg.replier_address {
            Some(address) => address.to_string(),
            None => bail!("RequestMessage.address is None"),
        };

        {
            let mut srv_connections = self.srv_connections.lock().unwrap();
            if srv_connections.insert(address.clone()) {
                self.requester.connect(&address)?;
                debug!("Connected to [{}] for service requests", address);
                sleep(Duration::from_millis(100));
            }
        }

        let my_requester_address = self.requester_address.lock().unwrap().clone();
        let messages = [
            zmq::Message::from(&msg.replier_id),
            zmq::Message::from(&msg.topic),
            zmq::Message::from(&my_requester_address),
            zmq::Message::from(&self.requester_id),
            zmq::Message::from(&msg.node_uuid),
            zmq::Message::from(&msg.req_uuid),
            zmq::Message::from(&msg.data),
            zmq::Message::from(&msg.req_type),
            zmq::Message::from(&msg.res_type),
        ];

        let mut v = VecDeque::from(messages);
        while let Some(zmq_msg) = v.pop_front() {
            let flag = if v.is_empty() { 0 } else { zmq::SNDMORE };
            self.requester.send(zmq_msg, flag)?;
        }

        Ok(())
    }

    pub(crate) fn reply(&self, msg: ReplyMessage) -> Result<()> {
        if let Some(sender) = self.reply_msg_sender.as_ref() {
            sender.send(msg)?;
            Ok(())
        } else {
            bail!("There is no sender. Transporter may not have started.");
        }
    }

    pub(crate) fn set_subscription_handler<F>(&mut self, handler: F)
    where
        F: Fn(PublishMessage) + Send + 'static,
    {
        self.subscription_handler
            .lock()
            .unwrap()
            .replace(Box::new(handler));
    }

    pub(crate) fn set_request_handler<F>(&mut self, handler: F)
    where
        F: Fn(RequestMessage) + Send + 'static,
    {
        self.request_handler
            .lock()
            .unwrap()
            .replace(Box::new(handler));
    }

    pub(crate) fn set_response_handler<F>(&mut self, handler: F)
    where
        F: Fn(ReplyMessage) + Send + 'static,
    {
        self.response_handler
            .lock()
            .unwrap()
            .replace(Box::new(handler));
    }

    pub(crate) fn start(&mut self) {
        let (subscribe_evt_sender, subscribe_evt_receiver) = mpsc::channel::<SubscribeEvent>();
        let (rep_msg_snd, reply_msg_receiver) = mpsc::channel::<ReplyMessage>();
        self.subscribe_evt_sender = Some(subscribe_evt_sender.clone());
        self.reply_msg_sender = Some(rep_msg_snd.clone());

        let context = self.context.clone();
        let host_addr = self.host_addr.clone();
        let requester_id = self.requester_id.clone();
        let requester_address = self.requester_address.clone();
        let replier_id = self.replier_id.clone();
        let replier_address = self.replier_address.clone();
        let connections = self.connections.clone();
        let srv_connections = self.srv_connections.clone();
        let subscription_handler = self.subscription_handler.clone();
        let request_handler = self.request_handler.clone();
        let response_handler = self.response_handler.clone();

        thread::spawn(move || {
            let mut inner = TransporterInner::new(
                context,
                &host_addr,
                &requester_id,
                requester_address,
                &replier_id,
                replier_address,
                connections,
                srv_connections,
                subscription_handler,
                request_handler,
                response_handler,
            );

            loop {
                if let Ok(event) = subscribe_evt_receiver.try_recv() {
                    if let Some(address) = event.connect {
                        if let Err(e) = inner.connect(&address) {
                            error!("failed connecting subscriber to [{}]: {}", address, e);
                        }
                    }
                    if let Some(address) = event.disconnect {
                        if let Err(e) = inner.disconnect(&address) {
                            error!("failed disconnecting subscriber from [{}]: {}", address, e);
                        }
                    }
                    if let Some(topic) = event.subscribe {
                        if let Err(e) = inner.set_subscribe(&topic) {
                            error!("failed subscribing to [{}]: {}", topic, e);
                        }
                    }
                    if let Some(topic) = event.unsubscribe {
                        if let Err(e) = inner.set_unsubscribe(&topic) {
                            error!("failed unsubscribing to [{}]: {}", topic, e);
                        }
                    }
                }

                if let Ok(msg) = reply_msg_receiver.try_recv() {
                    if let Err(e) = inner.reply(msg) {
                        error!("failed replying: {}", e);
                    }
                }

                inner.poll(TIMEOUT).expect("poll failed");
            }
        });
    }
}

struct TransporterInner {
    subscriber: zmq::Socket,
    response_receiver: zmq::Socket,
    replier: zmq::Socket,

    requester_id: String,
    requester_address: Arc<Mutex<String>>,
    replier_id: String,
    replier_address: Arc<Mutex<String>>,

    connections: Arc<Mutex<HashSet<String>>>,
    srv_connections: Arc<Mutex<HashSet<String>>>,

    subscription_handler: Arc<Mutex<Option<SubscriptionHandlerType>>>,
    request_handler: Arc<Mutex<Option<RequestHandlerType>>>,
    response_handler: Arc<Mutex<Option<ResponseHandlerType>>>,
}

impl TransporterInner {
    fn new(
        context: zmq::Context,
        host_addr: &str,
        requester_id: &str,
        requester_address: Arc<Mutex<String>>,
        replier_id: &str,
        replier_address: Arc<Mutex<String>>,
        connections: Arc<Mutex<HashSet<String>>>,
        srv_connections: Arc<Mutex<HashSet<String>>>,
        subscription_handler: Arc<Mutex<Option<SubscriptionHandlerType>>>,
        request_handler: Arc<Mutex<Option<RequestHandlerType>>>,
        response_handler: Arc<Mutex<Option<ResponseHandlerType>>>,
    ) -> Self {
        let any_tcp = format!("tcp://{}:*", host_addr);
        let requester_id = requester_id.to_string();
        let replier_id = replier_id.to_string();

        let subscriber = context.socket(zmq::SUB).unwrap();
        let response_receiver = context.socket(zmq::ROUTER).unwrap();
        let replier = context.socket(zmq::ROUTER).unwrap();

        subscriber.set_rcvhwm(DEFAULT_RCV_HWM).unwrap();

        response_receiver
            .set_identity(requester_id.as_bytes())
            .unwrap();
        response_receiver
            .bind(&any_tcp)
            .expect("failed binding response_receiver");
        let address = match response_receiver.get_last_endpoint() {
            Ok(Ok(endpoint)) => endpoint,
            _ => "error".to_string(),
        };
        *requester_address.lock().unwrap() = address.clone();

        replier.set_identity(replier_id.as_bytes()).unwrap();
        let linger_val = 0;
        replier.set_linger(linger_val).unwrap();
        let route_on = true;
        replier.set_router_mandatory(route_on).unwrap();
        replier.bind(&any_tcp).expect("failed binding replier");
        let address = match replier.get_last_endpoint() {
            Ok(Ok(endpoint)) => endpoint,
            _ => "".to_string(),
        };
        *replier_address.lock().unwrap() = address.clone();

        TransporterInner {
            subscriber,
            response_receiver,
            replier,
            requester_id,
            requester_address,
            replier_id,
            replier_address,
            connections,
            srv_connections,
            subscription_handler,
            request_handler,
            response_handler,
        }
    }

    fn connect(&self, address: &str) -> Result<()> {
        let mut connections = self.connections.lock().unwrap();
        if connections.insert(address.to_string()) {
            self.subscriber.connect(address)?;
            debug!("Connected to [{}] for subscriptions", address);
        } else {
            debug!("Already connected");
        }
        Ok(())
    }
    fn disconnect(&self, address: &str) -> Result<()> {
        self.connections.lock().unwrap().remove(address);
        self.subscriber.disconnect(&address)?;
        Ok(())
    }

    fn set_subscribe(&self, topic: &str) -> Result<()> {
        self.subscriber.set_subscribe(topic.as_bytes())?;
        Ok(())
    }

    fn set_unsubscribe(&self, topic: &str) -> Result<()> {
        self.subscriber.set_unsubscribe(topic.as_bytes())?;
        Ok(())
    }

    fn reply(&mut self, msg: ReplyMessage) -> Result<()> {
        let address = match msg.requester_address {
            Some(address) => address.to_string(),
            None => bail!("ReplyMessage.address is None"),
        };

        {
            let mut srv_connections = self.srv_connections.lock().unwrap();
            if srv_connections.insert(address.clone()) {
                self.replier.connect(&address)?;
                debug!("Connected to [{}] for service response", address);
                sleep(Duration::from_millis(100));
            }
        }

        let result_str = if msg.result { "1" } else { "0" };
        let messages = [
            zmq::Message::from(&msg.requester_id),
            zmq::Message::from(&msg.topic),
            zmq::Message::from(&msg.node_uuid),
            zmq::Message::from(&msg.req_uuid),
            zmq::Message::from(&msg.data),
            zmq::Message::from(result_str),
        ];

        let mut v = VecDeque::from(messages);
        while let Some(zmq_msg) = v.pop_front() {
            let flag = if v.is_empty() { 0 } else { zmq::SNDMORE };
            self.replier.send(zmq_msg, flag)?;
        }
        Ok(())
    }

    fn poll(&self, timeout: i64) -> Result<()> {
        let mut items = vec![
            self.subscriber.as_poll_item(zmq::POLLIN),
            self.replier.as_poll_item(zmq::POLLIN),
            self.response_receiver.as_poll_item(zmq::POLLIN),
        ];

        zmq::poll(&mut items, timeout)?;

        if items[0].is_readable() {
            self.on_subscribe()?;
        }
        if items[1].is_readable() {
            self.on_request()?;
        }
        if items[2].is_readable() {
            self.on_response()?;
        }

        Ok(())
    }

    fn on_subscribe(&self) -> Result<()> {
        let topic = self.subscriber.recv_msg(0)?;
        let address = self.subscriber.recv_msg(0)?;
        let data = self.subscriber.recv_msg(0)?;
        let msg_type = self.subscriber.recv_msg(0)?;

        if let Some(handler) = self.subscription_handler.lock().unwrap().as_ref() {
            handler(PublishMessage {
                topic: topic.as_str().unwrap_or("").to_string(),
                data: data.to_vec(),
                msg_type: msg_type.as_str().unwrap_or("").to_string(),
                publisher_address: address.as_str().unwrap_or("").to_string(),
            });
        }
        Ok(())
    }

    fn on_request(&self) -> Result<()> {
        let replier_id = self.replier.recv_msg(0)?;
        let topic = self.replier.recv_msg(0)?;
        let requester_address = self.replier.recv_msg(0)?;
        let requester_id = self.replier.recv_msg(0)?;
        let node_uuid = self.replier.recv_msg(0)?;
        let req_uuid = self.replier.recv_msg(0)?;
        let data = self.replier.recv_msg(0)?;
        let req_type = self.replier.recv_msg(0)?;
        let res_type = self.replier.recv_msg(0)?;

        if let Some(handler) = self.request_handler.lock().unwrap().as_ref() {
            handler(RequestMessage {
                replier_address: None,
                replier_id: replier_id.as_str().unwrap_or("").to_string(),
                topic: topic.as_str().unwrap_or("").to_string(),
                requester_address: requester_address.as_str().unwrap_or("").to_string(),
                requester_id: requester_id.as_str().unwrap_or("").to_string(),
                node_uuid: node_uuid.as_str().unwrap_or("").to_string(),
                req_uuid: req_uuid.as_str().unwrap_or("").to_string(),
                data: data.to_vec(),
                req_type: req_type.as_str().unwrap_or("").to_string(),
                res_type: res_type.as_str().unwrap_or("").to_string(),
            });
        }
        Ok(())
    }

    fn on_response(&self) -> Result<()> {
        let requester_id = self.response_receiver.recv_msg(0)?;
        let topic = self.response_receiver.recv_msg(0).unwrap();
        let node_uuid = self.response_receiver.recv_msg(0).unwrap();
        let req_uuid = self.response_receiver.recv_msg(0).unwrap();
        let data = self.response_receiver.recv_msg(0).unwrap();
        let result_str = self.response_receiver.recv_msg(0).unwrap();

        if let Some(handler) = self.response_handler.lock().unwrap().as_ref() {
            handler(ReplyMessage {
                requester_address: None,
                requester_id: requester_id.as_str().unwrap_or("").to_string(),
                topic: topic.as_str().unwrap_or("").to_string(),
                node_uuid: node_uuid.as_str().unwrap_or("").to_string(),
                req_uuid: req_uuid.as_str().unwrap_or("").to_string(),
                data: data.to_vec(),
                result: result_str.as_str().unwrap_or("0") == "1",
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;
    use std::io::Cursor;
    use std::time::SystemTime;

    const IP: &str = "127.0.0.1";
    const TOPIC: &str = "topic";

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Person {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub id: i32,
    }

    #[tokio::test]
    async fn test_pub_sub() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let mut transporter1 = Transporter::new(IP);
        let mut transporter2 = Transporter::new(IP);
        transporter1.start();
        let my_address = transporter1.publisher_address();
        let address = my_address.clone();

        let check = Arc::new(Mutex::new(None));
        let check2 = check.clone();

        let now = SystemTime::now();
        let now_clone = now.clone();

        tokio::spawn(async move {
            transporter2.start();
            transporter2.subscribe(Some(&address), None, Some(TOPIC), None);
            transporter2.set_subscription_handler(move |msg| {
                if let Ok(person) = Person::decode(&mut Cursor::new(msg.data)) {
                    check2.lock().unwrap().replace(person);

                    println!("elapsed: {:?}", now_clone.elapsed().unwrap());
                }
            });
            tokio::time::sleep(Duration::from_millis(100)).await;
        });

        tokio::time::sleep(Duration::from_millis(5)).await;

        let person = Person {
            name: "Alice".to_string(),
            id: 1234,
        };

        let mut data = person.encode_to_vec();
        transporter1
            .publish(PublishMessage {
                topic: TOPIC.to_string(),
                publisher_address: my_address.clone(),
                msg_type: "Person".to_string(),
                data,
            })
            .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;
        {
            let person = check.lock().unwrap().clone().unwrap();
            assert_eq!(person.name, "Alice");
            assert_eq!(person.id, 1234);
        }
    }
    #[tokio::test]
    async fn test_req_res() {
        // tracing_subscriber::fmt()
        //     .with_max_level(tracing::Level::DEBUG)
        //     .init();
        let mut transporter1 = Transporter::new(IP);
        let mut transporter2 = Transporter::new(IP);
        let replier_address = transporter2.replier_address.clone();
        let replier_id = transporter2.replier_id.clone();

        let now = SystemTime::now();
        let now_clone = now.clone();

        thread::spawn(move || {
            let (tx, mut rx) = mpsc::channel::<ReplyMessage>();

            transporter2.start();
            transporter2.set_request_handler(move |msg| {
                if let Ok(mut person) = Person::decode(&mut Cursor::new(msg.data)) {
                    person.name = "Bob".to_string();
                    tx.send(ReplyMessage {
                        requester_address: Some(msg.requester_address),
                        requester_id: msg.requester_id,
                        topic: msg.topic,
                        node_uuid: msg.node_uuid,
                        req_uuid: msg.req_uuid,
                        data: person.encode_to_vec(),
                        result: true,
                    })
                    .unwrap();
                }
            });

            if let Ok(msg) = rx.recv() {
                transporter2.reply(msg).unwrap();
                println!("req elapsed: {:?}", now_clone.elapsed().unwrap());
            }
            // sleep(Duration::from_millis(1000));
        });

        transporter1.start();
        tokio::time::sleep(Duration::from_millis(5)).await;
        let replier_address = replier_address.lock().unwrap().to_string();
        let check = Arc::new(Mutex::new(None));
        let check2 = check.clone();

        transporter1.set_response_handler(move |msg| {
            println!("res elapsed: {:?}", now.elapsed().unwrap());

            if let Ok(person) = Person::decode(&mut Cursor::new(msg.data)) {
                check2.lock().unwrap().replace(person);
            }
        });

        let person = Person {
            name: "Alice".to_string(),
            id: 1234,
        };

        let mut data = person.encode_to_vec();
        if let Err(e) = transporter1.request(RequestMessage {
            replier_address: Some(replier_address),
            replier_id,
            topic: TOPIC.to_string(),
            requester_address: "unset".to_string(),
            requester_id: "unset".to_string(),
            node_uuid: "node_uuid".to_string(),
            req_uuid: "req_uuid".to_string(),
            data,
            req_type: "Person".to_string(),
            res_type: "Person".to_string(),
        }) {
            println!("request failed: {}", e);
        }

        tokio::time::sleep(Duration::from_millis(1000)).await;

        {
            let person = check.lock().unwrap();
            // info!("person: {:?}", person);
            assert!(person.is_some());
            let person = person.as_ref().unwrap();
            assert_eq!(person.id, 1234);
            assert_eq!(person.name, "Bob");
        }
    }
}
