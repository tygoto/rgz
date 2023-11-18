use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use std::{env, io};
use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::time::{self, Duration, Instant};
use tracing::{debug, error, info, trace, warn};

use crate::discovery::store::DiscoveryStore;
use crate::discovery::{
    discovery_msg_decode, discovery_msg_encode, version, DEF_ACTIVITY_INTERVAL,
    DEF_HEARTBEAT_INTERVAL, DEF_SILENCE_INTERVAL, MAX_RCV_STR, TIMEOUT,
};
use crate::discovery::{
    DiscoveryDiscContents, DiscoveryFlags, DiscoveryMsg, DiscoveryPublisher, DiscoveryScope,
    DiscoverySubscriber, DiscoveryType,
};
use crate::utils::net as net_utils;

#[derive(Debug, PartialEq, Clone)]
pub(super) enum DestinationType {
    Unicast,
    Multicast,
    All,
}

#[derive(Clone)]
struct SendMsg {
    destination_type: DestinationType,
    discovery_type: DiscoveryType,
    discovery_publisher: DiscoveryPublisher,
}

type DiscoveryCallbackType = Box<dyn Fn(DiscoveryPublisher) + Send + Sync>;

pub(crate) struct Discovery {
    // Discovery wire protocol version.
    version: u32,

    // UDP socket used for sending/receiving discovery messages.
    socket: Option<Arc<UdpSocket>>,

    // Process UUID.
    p_uuid: String,

    // Host IP address.
    host_addr: Ipv4Addr,

    // List of host network interfaces.
    host_interfaces: Vec<Ipv4Addr>,

    // Address for sending to the multicast group.
    multicast_addr: SocketAddrV4,

    // Collection of socket addresses used as remote relays.
    relay_addrs: Arc<Mutex<HashSet<SocketAddrV4>>>,

    discovery_store: Arc<Mutex<DiscoveryStore>>,

    // Activity information. Every time there is a message from a
    // remote node, its activity information is updated. If we do not hear
    // from a node in a while, its entries in 'discovery_store' will be invalided.
    // The key is the process uuid.
    activity: Arc<Mutex<HashMap<String, Instant>>>,

    // Activity interval value (ms.).
    activity_interval: u64,

    // Heartbeat interval value (ms.).
    heartbeat_interval: u64,

    // Callback functions.
    connection_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
    disconnection_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
    registration_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
    unregistration_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,

    // Print discovery information to stdout.
    verbose: bool,

    msg_sender: Option<UnboundedSender<SendMsg>>,
}

impl Discovery {
    pub(crate) fn new(p_uuid: &str, ip: &str, port: u16, verbose: bool) -> Self {
        let p_uuid = p_uuid.to_string();
        let ip = ip.to_string();
        let multicast_group = ip.parse::<Ipv4Addr>().unwrap();
        let multicast_addr = SocketAddrV4::new(multicast_group.into(), port);
        let mut host_addr = Ipv4Addr::new(127, 0, 0, 1);
        if let Ok(host) = net_utils::determine_host() {
            host_addr = host;
        }

        // Get the list of network interfaces in this host.
        let mut host_interfaces: Vec<Ipv4Addr> = Vec::new();
        if let Ok(gz_ip) = env::var("GZ_IP") {
            if !gz_ip.is_empty() {
                let ip = gz_ip.parse::<Ipv4Addr>().unwrap();
                host_interfaces.push(ip);
            }
        } else {
            // Get the list of network interfaces in this host.
            if let Ok(net_ifaces) = net_utils::determine_interfaces() {
                host_interfaces = net_ifaces;
            }
        }

        let discovery_store = Arc::new(Mutex::new(DiscoveryStore::new()));

        Discovery {
            version: version(),
            socket: None,
            p_uuid,
            host_addr,
            host_interfaces,
            multicast_addr,
            relay_addrs: Arc::new(Mutex::new(Default::default())),
            discovery_store,
            activity: Arc::new(Mutex::new(Default::default())),
            activity_interval: DEF_ACTIVITY_INTERVAL,
            heartbeat_interval: DEF_HEARTBEAT_INTERVAL,
            connection_cb: Arc::new(Mutex::new(None)),
            disconnection_cb: Arc::new(Mutex::new(None)),
            registration_cb: Arc::new(Mutex::new(None)),
            unregistration_cb: Arc::new(Mutex::new(None)),
            verbose,
            msg_sender: None,
        }
    }

    pub(crate) fn set_connection_cb<F>(&mut self, callback: F)
    where
        F: Fn(DiscoveryPublisher) + Send + Sync + 'static,
    {
        self.connection_cb
            .lock()
            .unwrap()
            .replace(Box::new(callback));
    }

    pub(crate) fn set_disconnection_cb<F>(&mut self, callback: F)
    where
        F: Fn(DiscoveryPublisher) + Send + Sync + 'static,
    {
        self.disconnection_cb
            .lock()
            .unwrap()
            .replace(Box::new(callback));
    }

    pub(crate) fn set_registration_cb<F>(&mut self, callback: F)
    where
        F: Fn(DiscoveryPublisher) + Send + Sync + 'static,
    {
        self.registration_cb
            .lock()
            .unwrap()
            .replace(Box::new(callback));
    }

    pub(crate) fn set_unregistration_cb<F>(&mut self, callback: F)
    where
        F: Fn(DiscoveryPublisher) + Send + Sync + 'static,
    {
        self.unregistration_cb
            .lock()
            .unwrap()
            .replace(Box::new(callback));
    }

    pub fn advertise(&self, discovery_publisher: DiscoveryPublisher) -> Result<()> {
        {
            let mut store = self.discovery_store.lock().unwrap();
            if let Err(e) = store.add_publisher(discovery_publisher.clone()) {
                bail!("Failed to add publisher: {}", e);
            }
        }

        // Only advertise a message outside this process if the scope is not 'Process'
        let scope = DiscoveryScope::from_i32(discovery_publisher.scope);
        if scope.is_none() || scope == Some(DiscoveryScope::Process) {
            bail!("Only advertise a message if the scope is not 'Process'");
        }

        if let Some(sender) = self.msg_sender.as_ref() {
            sender.send(SendMsg {
                destination_type: DestinationType::All,
                discovery_type: DiscoveryType::Advertise,
                discovery_publisher,
            })?;
        }

        Ok(())
    }

    pub fn unadvertise(&self, topic: &str, n_uuid: &str) -> Result<()> {
        let mut publisher = None;

        {
            let mut store = self.discovery_store.lock().unwrap();
            if let Some(p) = store.publisher(topic, &self.p_uuid, n_uuid) {
                publisher = Some(p.clone());
            } else {
                warn!("Failed to find publisher");
                return Ok(());
            }
            store.del_publisher_by_node(topic, &self.p_uuid, n_uuid)?;
        }
        let publisher = publisher.unwrap();

        if publisher.scope == DiscoveryScope::Process as i32 {
            // Only unadvertise a message outside this process if the scope is not 'Process'.
            return Ok(());
        }

        // send msg
        if let Some(sender) = self.msg_sender.as_ref() {
            sender.send(SendMsg {
                destination_type: DestinationType::All,
                discovery_type: DiscoveryType::Unadvertise,
                discovery_publisher: publisher,
            })?;
        }

        Ok(())
    }

    pub fn discover(&self, topic: &str) -> Result<()> {
        if let Some(sender) = self.msg_sender.as_ref() {
            let publisher = DiscoveryPublisher {
                topic: topic.to_string(),
                process_uuid: self.p_uuid.to_string(),
                ..Default::default()
            };

            sender.send(SendMsg {
                destination_type: DestinationType::All,
                discovery_type: DiscoveryType::Subscribe,
                discovery_publisher: publisher,
            })?;

            let publishers = {
                let store = self.discovery_store.lock().unwrap();
                let mut v = vec![];
                for p in store.publishers(Some(topic), None, None) {
                    v.push(p.clone())
                }
                v
            };

            for p in publishers {
                if let Some(cb) = self.connection_cb.lock().unwrap().as_ref() {
                    cb(p.clone());
                }
            }
        }

        Ok(())
    }

    pub fn register(&self, publisher: DiscoveryPublisher) -> Result<()> {
        if let Some(sender) = self.msg_sender.as_ref() {
            sender.send(SendMsg {
                destination_type: DestinationType::All,
                discovery_type: DiscoveryType::NewConnection,
                discovery_publisher: publisher,
            })?;
        }
        Ok(())
    }

    pub fn unregister(&self, publisher: DiscoveryPublisher) -> Result<()> {
        if let Some(sender) = self.msg_sender.as_ref() {
            sender.send(SendMsg {
                destination_type: DestinationType::All,
                discovery_type: DiscoveryType::EndConnection,
                discovery_publisher: publisher.clone(),
            })?;
        }
        Ok(())
    }

    // Get all the publishers' information known for a given topic.
    pub fn publishers(&self, topic: &str) -> Option<Vec<DiscoveryPublisher>> {
        let store = self.discovery_store.lock().unwrap();
        let publishers: Vec<DiscoveryPublisher> = store
            .publishers(Some(topic), None, None)
            .into_iter()
            .cloned()
            .collect();

        if publishers.is_empty() {
            None
        } else {
            Some(publishers)
        }
    }

    // Get the list of topics currently advertised in the network.
    pub fn topic_list(&self) -> Vec<String> {
        self.discovery_store
            .lock()
            .unwrap()
            .topic_list()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn host_addr(&self) -> &Ipv4Addr {
        &self.host_addr
    }

    pub fn print_current_state(&self) {
        {
            let store = self.discovery_store.lock().unwrap();
            store.print();
        }
    }

    pub(crate) fn start(&mut self) {
        let (tx, mut rx) = unbounded_channel();
        self.msg_sender = Some(tx.clone());

        let mut heartbeat_interval = time::interval(Duration::from_millis(self.heartbeat_interval));
        let mut activity_interval = time::interval(Duration::from_millis(self.activity_interval));

        self.register_host_interfaces()
            .expect("Failed to register host interfaces.");
        let socket = self.socket.clone().unwrap();
        let inner = DiscoveryInner::new(
            self.version,
            self.p_uuid.clone(),
            socket.clone(),
            self.host_interfaces.clone(),
            self.multicast_addr.clone(),
            self.relay_addrs.clone(),
            self.activity.clone(),
            self.discovery_store.clone(),
            self.connection_cb.clone(),
            self.disconnection_cb.clone(),
            self.registration_cb.clone(),
            self.unregistration_cb.clone(),
            self.verbose,
        );

        tokio::spawn(async move {
            let mut rcv_str = vec![0u8; MAX_RCV_STR];
            loop {
                select! {
                    Some(msg) = rx.recv() => {
                        let snd_msg: SendMsg = msg.clone();
                        if let Err(err) = inner.send_msg(
                            snd_msg.destination_type,
                            snd_msg.discovery_type,
                            snd_msg.discovery_publisher).await
                        {
                            error!("Failed to send discovery message: {}", err);
                        }
                        if msg.discovery_type == DiscoveryType::Bye {
                            break;
                        }
                    }

                    Ok((len, addr)) = socket.recv_from(&mut rcv_str) => {
                        if let Err(err) = inner.recv_messages(rcv_str.clone(), len, addr).await {
                            error!("Failed to receive discovery message: {}", err);
                        }
                    }

                    _ = heartbeat_interval.tick() => {
                        if let Err(err) = inner.update_heartbeat().await {
                            error!("Failed to update heartbeat: {}", err);
                        }
                    }

                    _ = activity_interval.tick() => {
                        if let Err(err) = inner.update_activity().await {
                            error!("Failed to update activity: {}", err);
                        }
                    }
                }
            }
            debug!("Discovery thread finished. [{}]", inner.p_uuid);
        });
    }
    fn register_host_interfaces(&mut self) -> Result<()> {
        use socket2::{Domain, Protocol, Socket, Type};
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_read_timeout(Some(Duration::from_millis(TIMEOUT as u64)))?;
        socket.set_multicast_loop_v4(true)?;
        socket.set_reuse_address(true)?;
        socket.set_reuse_port(true)?;
        socket.set_nonblocking(true)?;

        for net_iface in self.host_interfaces.iter() {
            if let Err(_err) = socket.join_multicast_v4(self.multicast_addr.ip(), net_iface) {
                if net_iface == &self.host_addr {
                    let addr = "127.0.0.1:0".parse::<SocketAddrV4>().unwrap();
                    socket.join_multicast_v4(self.multicast_addr.ip(), addr.ip())?;
                    error!(
                        "Did you set the environment variable GZ_IP with a correct IP address? "
                    );
                    error!("  [{}] seems an invalid local IP address.", net_iface);
                    error!("  Using 127.0.0.1 as hostname.");
                    self.host_addr = *addr.ip();
                } else {
                    error!(
                        "Failed to join multicast group [{}] on interface [{}].",
                        self.multicast_addr.ip(),
                        net_iface
                    );
                }
            } else {
                debug!(
                    "Joining multicast group [{}] on interface [{}].",
                    self.multicast_addr.ip(),
                    net_iface
                );
            }
        }

        // Ipv4Addr::UNSPECIFIED.into()
        // Ipv4Addr::new(0, 0, 0, 0).into()
        // let address = SocketAddr::new(IpAddr::V4(self.host_addr), self.multicast_addr.port());
        let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), self.multicast_addr.port());
        socket.bind(&address.into())?;

        let socket = UdpSocket::from_std(socket.into())?;
        self.socket = Some(Arc::new(socket));

        Ok(())
    }
}

impl Drop for Discovery {
    fn drop(&mut self) {
        if let Some(sender) = self.msg_sender.as_ref() {
            if let Err(err) = sender.send(SendMsg {
                destination_type: DestinationType::All,
                discovery_type: DiscoveryType::Bye,
                discovery_publisher: DiscoveryPublisher {
                    process_uuid: self.p_uuid.clone(),
                    ..Default::default()
                },
            }) {
                debug!("Failed to send bye message: {}", err);
            }
        }
    }
}

pub(super) struct DiscoveryInner {
    version: u32,
    p_uuid: String,
    socket: Arc<UdpSocket>,
    host_interfaces: Vec<Ipv4Addr>,
    multicast_addr: SocketAddrV4,
    relay_addrs: Arc<Mutex<HashSet<SocketAddrV4>>>,
    activity: Arc<Mutex<HashMap<String, Instant>>>,
    discovery_store: Arc<Mutex<DiscoveryStore>>,
    connection_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
    disconnection_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
    registration_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
    unregistration_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,

    // Silence interval value (ms.).
    silence_interval: u64,
    verbose: bool,
}

impl DiscoveryInner {
    pub(super) fn new(
        version: u32,
        p_uuid: String,
        socket: Arc<UdpSocket>,
        host_interfaces: Vec<Ipv4Addr>,
        multicast_addr: SocketAddrV4,
        relay_addrs: Arc<Mutex<HashSet<SocketAddrV4>>>,
        activity: Arc<Mutex<HashMap<String, Instant>>>,
        discovery_store: Arc<Mutex<DiscoveryStore>>,
        connection_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
        disconnection_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
        registration_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
        unregistration_cb: Arc<Mutex<Option<DiscoveryCallbackType>>>,
        verbose: bool,
    ) -> Self {
        DiscoveryInner {
            version,
            p_uuid,
            socket,
            host_interfaces,
            multicast_addr,
            relay_addrs,
            activity,
            discovery_store,
            connection_cb,
            disconnection_cb,
            registration_cb,
            unregistration_cb,
            silence_interval: DEF_SILENCE_INTERVAL,
            verbose,
        }
    }
    fn is_local_ip(&self, from_ip: Ipv4Addr) -> bool {
        self.host_interfaces.iter().any(|&iface| iface == from_ip)
            || from_ip.to_string().starts_with("127.")
    }

    async fn recv_messages(
        &self,
        mut rcv_str: Vec<u8>,
        len: usize,
        addr: SocketAddr,
    ) -> Result<()> {
        if len == 0 {
            bail!("Received an empty discovery message.");
        }
        if addr.is_ipv6() {
            bail!("Received an IPv6 discovery message.");
        }
        let from_ip: Ipv4Addr = addr.ip().to_string().parse().unwrap();
        let msg = match discovery_msg_decode(&mut rcv_str, len) {
            Ok(msg) => msg,
            Err(e) => {
                bail!("Failed to decode discovery message: {}", e);
            }
        };

        if let Err(err) = self.dispatch_discovery_msg(msg, from_ip).await {
            error!("Failed to dispatch discovery message: {}", err);
        }

        Ok(())
    }
    async fn update_heartbeat(&self) -> Result<()> {
        self.send_msg(
            DestinationType::All,
            DiscoveryType::Heartbeat,
            DiscoveryPublisher {
                process_uuid: self.p_uuid.clone(),
                ..Default::default()
            },
        )
        .await?;

        // Re-advertise topics that are advertised inside this process.
        let publishers = {
            let store = self.discovery_store.lock().unwrap();
            let mut v = vec![];
            for p in store.publishers_by_process(&self.p_uuid) {
                v.push(p.clone())
            }
            v
        };

        for p in publishers {
            self.send_msg(DestinationType::All, DiscoveryType::Advertise, p)
                .await?;
        }

        Ok(())
    }

    async fn update_activity(&self) -> Result<()> {
        // Identify inactive processes and remove them from the activity map.
        let inactive_processes = {
            let mut activity = self.activity.lock().unwrap();
            let mut v = vec![];
            activity.retain(|process_uuid, last_activity| {
                if last_activity.elapsed().as_millis() > self.silence_interval as u128 {
                    debug!("Detect inactive process: {}", process_uuid);
                    v.push(process_uuid.clone());
                    false
                } else {
                    true
                }
            });
            v
        };

        // Remove publishers related to inactive processes.
        let disconnect_processes = {
            let mut store = self.discovery_store.lock().unwrap();
            inactive_processes
                .into_iter()
                .filter(|process_uuid| {
                    if let Err(err) = store.del_publishers_by_process(process_uuid) {
                        debug!("Failed to delete publishers by process: {}", err);
                        false
                    } else {
                        true
                    }
                })
                .collect::<Vec<_>>()
        };

        // Trigger the disconnection callback.
        if let Some(cb) = self.disconnection_cb.lock().unwrap().as_ref() {
            for process_uuid in &disconnect_processes {
                let publisher = DiscoveryPublisher {
                    process_uuid: process_uuid.clone(),
                    ..Default::default()
                };
                cb(publisher);
            }
        }

        Ok(())
    }

    async fn dispatch_discovery_msg(&self, mut msg: DiscoveryMsg, from_ip: Ipv4Addr) -> Result<()> {
        // "Received a discovery message with a different version number."
        if self.version != msg.version {
            return Ok(());
        }
        // Discard our own discovery messages.
        if self.p_uuid == msg.process_uuid {
            return Ok(());
        }
        trace!("Received [{}] from [{}].", msg.r#type, msg.process_uuid);

        // Forwarding summary:
        //   - From a unicast peer  -> to multicast group (with NO_RELAY flag).
        //   - From multicast group -> to unicast peers (with RELAY flag).

        let flags = msg.flags;
        if let Some(flag) = flags.as_ref() {
            if flag.relay {
                msg.flags = Some(DiscoveryFlags {
                    relay: false,
                    no_relay: true,
                });

                self.send_multicast(&msg).await?;

                // A unicast peer contacted me. I need to save its address for
                // sending future messages in the future.
                {
                    let mut relay_addrs = self.relay_addrs.lock().unwrap();
                    let addr = SocketAddrV4::new(from_ip, self.multicast_addr.port());
                    relay_addrs.insert(addr);
                }
                return Ok(());
            } else if !flag.no_relay {
                msg.flags = Some(DiscoveryFlags {
                    relay: true,
                    no_relay: false,
                });
                self.send_unicast(&msg).await?;
            }
        } else {
            msg.flags = Some(DiscoveryFlags {
                relay: true,
                no_relay: false,
            });
            self.send_unicast(&msg).await?;
        }

        {
            let mut activity = self.activity.lock().unwrap();
            activity.insert(msg.process_uuid.clone(), Instant::now());
        }

        let mut publisher = match msg.disc_contents {
            Some(DiscoveryDiscContents::Pub(ref publisher)) => publisher.clone(),
            _ => DiscoveryPublisher {
                ..Default::default()
            },
        };

        let is_sender_local = self.is_local_ip(from_ip);

        let msg_type = DiscoveryType::from_i32(msg.r#type).unwrap_or(DiscoveryType::Uninitialized);

        // debug!("Received [{}] from [{}].", &msg_type.as_str_name(), &msg.process_uuid);
        match msg_type {
            DiscoveryType::Advertise => {
                // Check scope of the topic.
                if let Ok(scope) = DiscoveryScope::try_from(publisher.scope) {
                    if scope == DiscoveryScope::Process
                        || (scope == DiscoveryScope::Host && !is_sender_local)
                    {
                        return Ok(()); // Discard the message.
                    }
                }

                // Register an advertised address for the topic.
                match self
                    .discovery_store
                    .lock()
                    .unwrap()
                    .add_publisher(publisher.clone())
                {
                    Ok(_) => {
                        if let Some(cb) = self.connection_cb.lock().unwrap().as_ref() {
                            cb(publisher.clone());
                        }
                    }
                    Err(_e) => {
                        // debug!("{:?}", _e);
                    }
                }
            }

            DiscoveryType::Subscribe => {
                if let Some(DiscoveryDiscContents::Sub(subscriber)) = &msg.disc_contents {
                    let recv_topic = &subscriber.topic;

                    // Get registered publishers from the same process.
                    let publishers = {
                        let mut v = vec![];
                        let store = self.discovery_store.lock().unwrap();
                        for p in store.publishers(Some(recv_topic), Some(&msg.process_uuid), None) {
                            if let Some(scope) = DiscoveryScope::from_i32(p.scope) {
                                if scope == DiscoveryScope::Process
                                    || (scope == DiscoveryScope::Host && !is_sender_local)
                                {
                                    continue;
                                }
                                v.push(p.clone());
                            }
                        }
                        v
                    };
                    for publisher in publishers {
                        self.send_msg(DestinationType::All, DiscoveryType::Advertise, publisher)
                            .await?;
                    }
                } else {
                    bail!("Subscription discovery message is missing Subscriber information.");
                }
            }
            DiscoveryType::NewConnection => {
                if let Some(cb) = self.registration_cb.lock().unwrap().as_ref() {
                    cb(publisher.clone());
                }
            }
            DiscoveryType::EndConnection => {
                if let Some(cb) = self.unregistration_cb.lock().unwrap().as_ref() {
                    cb(publisher.clone());
                }
            }
            DiscoveryType::Heartbeat => {
                // The timestamp has already been updated.
            }
            DiscoveryType::Bye => {
                // Remove the activity entry for this publisher.
                {
                    let mut activity = self.activity.lock().unwrap();
                    activity.remove(&msg.process_uuid);
                }
                // Notify the new disconnection.
                publisher.process_uuid = msg.process_uuid.clone();

                if let Some(cb) = self.disconnection_cb.lock().unwrap().as_ref() {
                    cb(publisher.clone());
                }

                // Remove the address entry for this topic.
                {
                    let mut store = self.discovery_store.lock().unwrap();
                    if let Err(e) = store.del_publishers_by_process(&msg.process_uuid) {
                        debug!("Failed to remove publisher: {}", e);
                    }
                }
            }
            DiscoveryType::Unadvertise => {
                if let Some(scope) = DiscoveryScope::from_i32(publisher.scope) {
                    if scope == DiscoveryScope::Process
                        || (scope == DiscoveryScope::Host && !is_sender_local)
                    {
                        return Ok(()); // Discard the message.
                    }
                }

                if let Some(cb) = self.disconnection_cb.lock().unwrap().as_ref() {
                    cb(publisher.clone());
                }

                {
                    let mut store = self.discovery_store.lock().unwrap();
                    if let Err(e) = store.del_publisher_by_node(
                        &publisher.topic,
                        &publisher.process_uuid,
                        &publisher.node_uuid,
                    ) {
                        debug!("Failed to remove publisher: {:?}", e);
                    }
                }
            }
            _ => {
                bail!("Unknown message type [{}]", msg_type.as_str_name());
            }
        }

        Ok(())
    }
    async fn send_msg(
        &self,
        destination_type: DestinationType,
        discovery_type: DiscoveryType,
        publisher: DiscoveryPublisher,
    ) -> Result<()> {
        let mut discovery_msg = DiscoveryMsg {
            version: self.version,
            process_uuid: self.p_uuid.clone(),
            ..Default::default()
        };
        discovery_msg.set_type(discovery_type);

        match discovery_type {
            DiscoveryType::Advertise
            | DiscoveryType::Unadvertise
            | DiscoveryType::NewConnection
            | DiscoveryType::EndConnection => {
                let disc_contents = DiscoveryDiscContents::Pub(publisher.clone());
                discovery_msg.disc_contents = Some(disc_contents);
            }
            DiscoveryType::Subscribe => {
                let subscriber = DiscoverySubscriber {
                    topic: publisher.topic.clone(),
                };
                let disc_contents = DiscoveryDiscContents::Sub(subscriber);
                discovery_msg.disc_contents = Some(disc_contents);
            }
            DiscoveryType::Heartbeat | DiscoveryType::Bye => {}
            _ => {
                bail!(
                    "Discovery::SendMsg() error: Unrecognized message type [{:?}]",
                    discovery_type.as_str_name()
                );
            }
        }

        if destination_type == DestinationType::Multicast
            || destination_type == DestinationType::All
        {
            self.send_multicast(&discovery_msg).await?;
        }

        if destination_type == DestinationType::Unicast || destination_type == DestinationType::All
        {
            let flags = DiscoveryFlags {
                relay: true,
                no_relay: false,
            };
            discovery_msg.flags = Some(flags);
            self.send_unicast(&discovery_msg).await?;
        }

        if self.verbose {
            info!(
                "\t* Sending: {} topic [{}] in p_uuid[{}]",
                discovery_type.as_str_name(),
                publisher.topic,
                self.p_uuid
            );
        }

        Ok(())
    }

    async fn send_unicast(&self, msg: &DiscoveryMsg) -> Result<()> {
        let addrs = {
            self.relay_addrs
                .lock()
                .unwrap()
                .iter()
                .map(|x| x.clone())
                .collect::<Vec<_>>()
        };

        if let Ok((buffer, total_size)) = discovery_msg_encode(&msg) {
            for sock_addr in addrs {
                match self.socket.send_to(&buffer, sock_addr).await {
                    Ok(sent) if sent != total_size => {
                        eprintln!("Exception sending a unicast message:");
                        eprintln!("  Return value: {}", sent);
                        eprintln!("  Error code: {}", io::Error::last_os_error());
                        break;
                    }
                    Err(err) => {
                        eprintln!("Exception sending a unicast message: {}", err);
                        break;
                    }
                    _ => {}
                }
            }
        } else {
            bail!("Discovery::SendUnicast: Error serializing data.")
        }

        Ok(())
    }
    async fn send_multicast(&self, msg: &DiscoveryMsg) -> Result<()> {
        if let Ok((buffer, total_size)) = discovery_msg_encode(&msg) {
            // TODO: Support multiple Sockets.
            // for sock in self.sockets.iter() { }
            match self.socket.send_to(&buffer, self.multicast_addr).await {
                Ok(sent) if sent != total_size => {
                    eprintln!(
                        "Exception sending a multicast message: {}",
                        io::Error::last_os_error()
                    );
                    // break;
                }
                Err(err) => {
                    eprintln!("Exception sending a multicast message: {}", err);
                    // break;
                }
                _ => {}
            }
        } else {
            bail!("Discovery::SendMulticast: Error serializing data.")
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use tracing_subscriber;
    use uuid::uuid;

    use super::*;

    const MSG_PORT: u16 = 11319;
    const SUB_PORT: u16 = 11320;
    const IP: &str = "224.0.0.7";
    const TOPIC: &str = "test_topic";
    const SERVICE_NAME: &str = "test_service";
    const ADDR1: &str = "tcp://127.0.0.1:12345";
    const CTRL1: &str = "tcp://127.0.0.1:12346";
    const P_UUID1: Lazy<String> =
        Lazy::new(|| uuid!("00000000-0000-0000-0000-000000000011").to_string());
    const N_UUID1: Lazy<String> =
        Lazy::new(|| uuid!("00000000-0000-0000-0000-000000000012").to_string());
    const ADDR2: &str = "tcp://127.0.0.1:12347";
    const CTRL2: &str = "tcp://127.0.0.1:12348";
    const P_UUID2: Lazy<String> =
        Lazy::new(|| uuid!("00000000-0000-0000-0000-000000000021").to_string());
    const N_UUID2: Lazy<String> =
        Lazy::new(|| uuid!("00000000-0000-0000-0000-000000000022").to_string());

    // Try to use the discovery features without calling start().
    #[tokio::test]
    async fn test_without_calling_start() {
        let mut discovery = Discovery::new(&P_UUID1, IP, MSG_PORT, true);

        let result = discovery.advertise(DiscoveryPublisher {
            topic: TOPIC.to_string(),
            address: ADDR1.to_string(),
            process_uuid: P_UUID1.to_string(),
            node_uuid: N_UUID1.to_string(),
            scope: DiscoveryScope::All as i32,
            ..Default::default()
        });
        assert!(result.is_ok());

        let result = discovery.discover(TOPIC);
        assert!(result.is_ok());

        let result = discovery.unadvertise(TOPIC, &P_UUID1);
        assert!(result.is_ok());
    }

    // Advertise a topic
    #[tokio::test]
    async fn test_advertise() {
        // tracing_subscriber::fmt()
        //     .with_max_level(tracing::Level::DEBUG)
        //     .init();

        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);
        let mut discovery2 = Discovery::new(&P_UUID2, IP, MSG_PORT, true);

        discovery1.start();
        discovery2.start();

        time::sleep(Duration::from_millis(100)).await;

        discovery1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: None,
            })
            .unwrap();

        time::sleep(Duration::from_millis(100)).await;

        {
            let store = discovery2.discovery_store.lock().unwrap();
            let publisher = store.publisher(TOPIC, &P_UUID1, &N_UUID1);
            assert!(publisher.is_some());
        }
    }

    // Check that the discovery triggers the callbacks after an advertise.
    #[tokio::test]
    async fn test_advertise_with_callback() {
        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);
        let mut discovery2 = Discovery::new(&P_UUID2, IP, MSG_PORT, true);

        let check = Arc::new(Mutex::new(false));
        let check1 = check.clone();
        discovery2.set_connection_cb(move |publisher| {
            let mut c = check1.lock().unwrap();
            *c = publisher.process_uuid == P_UUID1.to_string();
        });

        discovery1.start();
        discovery2.start();

        discovery1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: None,
            })
            .unwrap();

        time::sleep(Duration::from_millis(100)).await;

        {
            let c = check.lock().unwrap();
            assert!(*c);
        }

        {
            let mut c = check.lock().unwrap();
            *c = false;
        }

        discovery1
            .advertise(DiscoveryPublisher {
                topic: "topic2".to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::Host as i32,
                pub_type: None,
            })
            .unwrap();

        time::sleep(Duration::from_millis(100)).await;
        {
            let c = check.lock().unwrap();
            assert!(*c);
        }
    }
    //
    #[tokio::test]
    async fn test_advertise_same_process() {
        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);
        let mut discovery2 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);

        let check = Arc::new(Mutex::new(false));
        let check1 = check.clone();
        discovery2.set_connection_cb(move |publisher| {
            let mut c = check1.lock().unwrap();
            *c = publisher.process_uuid == P_UUID1.to_string();
        });

        discovery1.start();
        discovery2.start();

        discovery1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: None,
            })
            .unwrap();

        time::sleep(Duration::from_millis(100)).await;
        {
            let c = check.lock().unwrap();
            assert_eq!(*c, false);
        }
    }

    // Check that the discovery triggers the callbacks after a discovery
    // and after register the discovery callback.
    #[tokio::test]
    async fn test_discovery() {
        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);
        let mut discovery2 = Discovery::new(&P_UUID2, IP, MSG_PORT, true);
        let check = Arc::new(Mutex::new(false));

        discovery1.start();
        discovery2.start();

        discovery1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: None,
            })
            .unwrap();
        time::sleep(Duration::from_millis(100)).await;

        {
            let c = check.lock().unwrap();
            assert_eq!(*c, false);
        }

        let check1 = check.clone();
        discovery2.set_connection_cb(move |publisher| {
            let mut c = check1.lock().unwrap();
            *c = publisher.process_uuid == P_UUID1.to_string();
        });

        discovery2.discover(TOPIC).unwrap();

        time::sleep(Duration::from_millis(100)).await;

        {
            let c = check.lock().unwrap();
            assert_eq!(*c, true);
        }
    }

    // Check that the discovery triggers the disconnection callback after
    // an unadvertise.
    #[tokio::test]
    async fn test_unadvertise() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);
        let mut discovery2 = Discovery::new(&P_UUID2, IP, MSG_PORT, true);
        let check = Arc::new(Mutex::new(false));
        let check1 = check.clone();
        discovery2.set_disconnection_cb(move |publisher| {
            let mut c = check1.lock().unwrap();
            *c = publisher.process_uuid == P_UUID1.to_string();
        });

        discovery1.start();
        discovery2.start();

        discovery1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: None,
            })
            .unwrap();

        time::sleep(Duration::from_millis(100)).await;
        {
            let c = check.lock().unwrap();
            assert_eq!(*c, false);
        }

        discovery1.unadvertise(TOPIC, &N_UUID1).unwrap();

        time::sleep(Duration::from_millis(100)).await;

        {
            let c = check.lock().unwrap();
            assert_eq!(*c, true);
        }
    }

    // Check that the discovery triggers the disconnection callback after
    // sending a BYE message (discovery object out of scope).
    #[tokio::test]
    async fn test_bye() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);
        let mut discovery2 = Discovery::new(&P_UUID2, IP, MSG_PORT, true);
        let check = Arc::new(Mutex::new(false));
        let check1 = check.clone();
        discovery2.set_disconnection_cb(move |publisher| {
            let mut c = check1.lock().unwrap();
            *c = publisher.process_uuid == P_UUID1.to_string();
        });

        discovery1.start();
        discovery2.start();

        discovery1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: None,
            })
            .unwrap();

        time::sleep(Duration::from_millis(100)).await;

        drop(discovery1);

        time::sleep(Duration::from_millis(4000)).await;

        {
            let activity = discovery2.activity.lock().unwrap();
            assert_eq!(activity.len(), 0);
        }

        {
            let c = check.lock().unwrap();
            assert_eq!(*c, true);
        }
    }

    // Check that the discovery detects two publishers advertising the same
    // topic name.
    #[tokio::test]
    async fn test_two_publishers_same_topic() {
        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);
        let mut discovery2 = Discovery::new(&P_UUID2, IP, MSG_PORT, true);

        let counter = Arc::new(Mutex::new(0));
        let c = counter.clone();
        discovery2.set_connection_cb(move |publisher| {
            let mut c = c.lock().unwrap();
            *c += 1;
        });

        let publisher1 = DiscoveryPublisher {
            topic: TOPIC.to_string(),
            address: ADDR1.to_string(),
            process_uuid: P_UUID1.to_string(),
            node_uuid: N_UUID1.to_string(),
            scope: DiscoveryScope::All as i32,
            pub_type: None,
        };

        let publisher2 = DiscoveryPublisher {
            topic: TOPIC.to_string(),
            address: ADDR2.to_string(),
            process_uuid: P_UUID2.to_string(),
            node_uuid: N_UUID2.to_string(),
            scope: DiscoveryScope::All as i32,
            pub_type: None,
        };

        discovery1.start();
        discovery2.start();

        let result = discovery1.advertise(publisher1.clone());
        assert!(result.is_ok());
        let result = discovery2.advertise(publisher2.clone());
        assert!(result.is_ok());

        time::sleep(Duration::from_millis(100)).await;
        {
            let mut c = counter.lock().unwrap();
            assert_eq!(*c, 1);
            *c = 0;
        }

        discovery2.discover(TOPIC).unwrap();

        time::sleep(Duration::from_millis(100)).await;
        {
            let c = counter.lock().unwrap();
            assert_eq!(*c, 2);
        }
    }

    // Check that a discovery service sends messages if there are
    // topics or services advertised in its process.
    #[tokio::test]
    async fn test_discovery_service() {
        // tracing_subscriber::fmt()
        //     .with_max_level(tracing::Level::DEBUG)
        //     .init();

        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);

        discovery1.start();

        let mut discovery2 = Discovery::new(&P_UUID2, IP, MSG_PORT, true);

        discovery2.start();
        discovery1
            .advertise(DiscoveryPublisher {
                topic: TOPIC.to_string(),
                address: ADDR1.to_string(),
                process_uuid: P_UUID1.to_string(),
                node_uuid: N_UUID1.to_string(),
                scope: DiscoveryScope::All as i32,
                pub_type: None,
            })
            .unwrap();

        time::sleep(Duration::from_millis(100)).await;

        {
            let activity = discovery1.activity.lock().unwrap();
            assert!(activity
                .iter()
                .any(|(process_uuid, _)| process_uuid == &P_UUID2.to_string()));
        }

        {
            let activity = discovery2.activity.lock().unwrap();
            assert!(activity
                .iter()
                .any(|(process_uuid, _)| process_uuid == &P_UUID1.to_string()));
        }

        drop(discovery2);

        time::sleep(Duration::from_millis(3000)).await;

        {
            let activity = discovery1.activity.lock().unwrap();
            assert_eq!(
                activity
                    .iter()
                    .any(|(process_uuid, _)| process_uuid == &P_UUID2.to_string()),
                false
            );
        }
    }

    // Check that a wrong GZ_IP value makes HostAddr() to return 127.0.0.1
    #[tokio::test]
    async fn test_gz_ip() {
        let mut gz_ip = None;
        if let Ok(ip) = env::var("GZ_IP") {
            gz_ip = Some(ip.clone());
        }
        env::set_var("GZ_IP", "127.0.0.1");

        let mut discovery1 = Discovery::new(&P_UUID1, IP, MSG_PORT, true);

        assert_eq!(discovery1.host_addr.to_string(), "127.0.0.1");

        if let Some(ip) = gz_ip {
            env::set_var("GZ_IP", ip);
        } else {
            env::set_var("GZ_IP", "");
        }
    }
}
