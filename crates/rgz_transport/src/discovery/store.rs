use anyhow::{bail, Result};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::discovery::{DiscoveryPubType, DiscoveryPublisher};

pub(crate) struct DiscoveryStore {
    discovery_publishers: HashMap<String, HashMap<String, Vec<DiscoveryPublisher>>>,
}

impl DiscoveryStore {
    pub(super) fn new() -> Self {
        Self {
            discovery_publishers: HashMap::new(),
        }
    }

    /// Add a new address associated to a given topic and node UUID.
    pub fn add_publisher(&mut self, discovery_publisher: DiscoveryPublisher) -> Result<()> {
        let processes = self
            .discovery_publishers
            .entry(discovery_publisher.topic.to_string())
            .or_insert(HashMap::new());

        if let Some(publishers) = processes.get(&discovery_publisher.process_uuid) {
            if publishers.iter().any(|p| {
                p.address == discovery_publisher.address
                    && p.node_uuid == discovery_publisher.node_uuid
            }) {
                bail!("Publisher already exists");
            }
        }
        processes
            .entry(discovery_publisher.process_uuid.to_string())
            .or_insert_with(Vec::new)
            .push(discovery_publisher);

        Ok(())
    }
    /// Remove a publisher associated to a given topic and UUID pair.
    pub fn del_publisher_by_node(&mut self, topic: &str, p_uuid: &str, n_uuid: &str) -> Result<()> {
        if let Some(processes) = self.discovery_publishers.get_mut(topic) {
            if let Some(publishers) = processes.get_mut(p_uuid) {
                if !publishers.iter().any(|p| p.node_uuid == n_uuid) {
                    bail!("Publisher not found");
                }
                publishers.retain(|p| p.node_uuid != n_uuid);
                if publishers.is_empty() {
                    processes.remove(p_uuid);
                }
                if processes.is_empty() {
                    self.discovery_publishers.remove(topic);
                }
                return Ok(());
            }
        }
        bail!("Publisher not found");
    }

    /// Remove all the publishers associated to a given process.
    pub fn del_publishers_by_process(&mut self, p_uuid: &str) -> Result<()> {
        let mut del_check = false;
        let mut topics_to_remove = Vec::new();
        for (topic, processes) in &mut self.discovery_publishers {
            if processes.remove(p_uuid).is_some() {
                del_check = true;
                if processes.is_empty() {
                    topics_to_remove.push(topic.to_string());
                }
            }
        }
        if !del_check {
            bail!("Publisher not found");
        }
        for topic in topics_to_remove {
            self.discovery_publishers.remove(&topic);
        }
        Ok(())
    }

    /// Return if there is any publisher stored for the given topic.
    pub fn has_topic(&self, topic: &str) -> bool {
        self.discovery_publishers.contains_key(topic)
    }

    /// Return if there is any publisher stored for the given topic and type.
    pub fn has_topic_and_msg_type(&self, topic: &str, msg_type: &str) -> bool {
        match self.discovery_publishers.get(topic) {
            Some(processes) => processes.values().any(|v| {
                v.iter().any(|p| {
                    p.pub_type
                        .as_ref()
                        .map(|pub_type| match pub_type {
                            DiscoveryPubType::MsgPub(publisher) => publisher.msg_type == msg_type,
                            _ => false,
                        })
                        .unwrap_or(false)
                })
            }),
            None => false,
        }
    }

    /// Return if there is any publisher stored for the given topic and process UUID.
    pub fn has_any_publishers(&self, topic: &str, p_uuid: &str) -> bool {
        match self.discovery_publishers.get(topic) {
            Some(publishers) => publishers.contains_key(p_uuid),
            None => false,
        }
    }

    /// Return if the requested publisher's address is stored.
    pub fn has_publisher(&self, addr: &str) -> bool {
        self.discovery_publishers
            .values()
            .any(|v| v.values().any(|v| v.iter().any(|p| p.address == addr)))
    }

    /// Get the address information for a given topic and node UUID.
    pub fn publisher(
        &self,
        topic: &str,
        p_uuid: &str,
        n_uuid: &str,
    ) -> Option<&DiscoveryPublisher> {
        match self.discovery_publishers.get(topic) {
            Some(publishers) => match publishers.get(p_uuid) {
                Some(publishers) => publishers.iter().find(|p| p.node_uuid == n_uuid),
                None => None,
            },
            None => None,
        }
    }

    pub fn publishers(
        &self,
        topic: Option<&str>,
        p_uuid: Option<&str>,
        n_uuid: Option<&str>,
    ) -> Vec<&DiscoveryPublisher> {
        match (topic.is_some(), p_uuid.is_some(), n_uuid.is_some()) {
            (true, true, true) => self
                .publisher(topic.unwrap(), p_uuid.unwrap(), n_uuid.unwrap())
                .map(|p| vec![p])
                .unwrap_or(vec![]),
            (true, true, false) => match self.discovery_publishers.get(topic.unwrap()) {
                Some(process) => process
                    .get(p_uuid.unwrap())
                    .map(|p| p.iter().collect())
                    .unwrap_or(vec![]),
                None => vec![],
            },
            (true, false, false) => self.publishers_by_topic(topic.unwrap()),
            (true, false, true) => match self.discovery_publishers.get(topic.unwrap()) {
                Some(process) => process
                    .values()
                    .flat_map(|p| p.iter())
                    .filter(|p| p.node_uuid == n_uuid.unwrap())
                    .collect(),
                None => vec![],
            },
            (false, true, false) => self.publishers_by_process(p_uuid.unwrap()),
            (false, true, true) => self.publishers_by_node(p_uuid.unwrap(), n_uuid.unwrap()),
            (false, false, true) => self
                .discovery_publishers
                .values()
                .flat_map(|p| p.values())
                .flatten()
                .filter(|p| p.node_uuid == n_uuid.unwrap())
                .collect(),
            (false, false, false) => self
                .discovery_publishers
                .values()
                .flat_map(|p| p.values())
                .flatten()
                .collect(),
        }
    }

    /// Get the map of publishers stored for a given topic.
    pub fn publishers_by_topic(&self, topic: &str) -> Vec<&DiscoveryPublisher> {
        match self.discovery_publishers.get(topic) {
            Some(publishers) => publishers.values().flatten().collect(),
            None => vec![],
        }
    }

    /// Given a process UUID, the function returns the list of
    /// publishers contained in this process UUID with its address information
    pub fn publishers_by_process(&self, p_uuid: &str) -> Vec<&DiscoveryPublisher> {
        self.discovery_publishers
            .values()
            .flat_map(|process| process.get(p_uuid))
            .flatten()
            .collect()
    }

    /// Given a process UUID and the node UUID, the function returns
    /// the list of publishers contained in the node.
    pub fn publishers_by_node(&self, p_uuid: &str, n_uuid: &str) -> Vec<&DiscoveryPublisher> {
        self.discovery_publishers
            .values()
            .flat_map(|process| process.get(p_uuid))
            .flatten()
            .filter(|p| p.node_uuid == n_uuid)
            .collect()
    }

    /// Get the list of topics currently stored.
    pub fn topic_list(&self) -> Vec<&str> {
        self.discovery_publishers
            .keys()
            .map(|k| k.as_str())
            .collect()
    }

    /// Print all the information for debugging purposes.
    pub fn print(&self) {
        let mut collect: Vec<_> = self.discovery_publishers.iter().collect();
        collect.sort_by(|a, b| a.0.cmp(&b.0));
        collect.iter().for_each(|(topic, processes)| {
            processes.iter().for_each(|(p_uuid, publishers)| {
                publishers.iter().for_each(|p| {
                    Self::print_publisher(p);
                });
            });
        });
    }
    pub fn print_publisher(publisher: &DiscoveryPublisher) {
        println!("Publisher:");
        println!("\ttopic: {}", publisher.topic);
        println!("\taddress: {}", publisher.address);
        println!("\tprocess_uuid: {}", publisher.process_uuid);
        println!("\tnode_uuid: {}", publisher.node_uuid);
        if let Some(pub_type) = &publisher.pub_type {
            match pub_type {
                DiscoveryPubType::MsgPub(msg_pub) => {
                    println!("\tpub_type: MsgPub");
                    println!("\t\tctrl: {}", msg_pub.ctrl);
                    println!("\t\tthrottled: {}", msg_pub.throttled);
                    println!("\t\tmsgs_per_sec: {}", msg_pub.msgs_per_sec);
                    println!("\t\tmsg_type: {}", msg_pub.msg_type);
                }
                DiscoveryPubType::SrvPub(srv_pub) => {
                    println!("\tpub_type: SrvPub:");
                    println!("\t\tsocket_id: {}", srv_pub.socket_id);
                    println!("\t\trequest_type: {}", srv_pub.request_type);
                    println!("\t\tresponse_type: {}", srv_pub.response_type);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::{DiscoveryMsgPublisher, DiscoveryScope};

    fn create_msg_publisher(
        topic: &str,
        addr: &str,
        p_uuid: &str,
        n_uuid: &str,
        scope: DiscoveryScope,
        msg_type: &str,
    ) -> DiscoveryPublisher {
        DiscoveryPublisher {
            topic: topic.to_string(),
            address: addr.to_string(),
            process_uuid: p_uuid.to_string(),
            node_uuid: n_uuid.to_string(),
            scope: scope as i32,
            pub_type: Some(DiscoveryPubType::MsgPub(DiscoveryMsgPublisher {
                ctrl: "".to_string(),
                throttled: false,
                msgs_per_sec: 0,
                msg_type: msg_type.to_string(),
            })),
        }
    }

    #[test]
    fn test_add_publisher() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let check = store.add_publisher(pub1.clone());
        assert_eq!(check.unwrap(), ());

        let check = store.add_publisher(pub1.clone());
        assert_eq!(check.unwrap_err().to_string(), "Publisher already exists");
    }

    #[test]
    fn test_has_topic() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();

        assert_eq!(store.has_topic("topic1"), true);
        assert_eq!(store.has_topic("topic2"), false);
    }

    #[test]
    fn test_has_topic_and_msg_type() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        let result = store.has_topic_and_msg_type("topic1", "gz.msgs.StringMsg");
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_any_publishers() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        assert_eq!(store.has_any_publishers("topic1", "p_uuid1"), true);
        assert_eq!(store.has_any_publishers("topic1", "p_uuid2"), false);
    }

    #[test]
    fn test_has_publisher() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );
        store.add_publisher(pub1.clone()).unwrap();
        assert_eq!(store.has_publisher("addr1"), true);
        assert_eq!(store.has_publisher("addr2"), false);
    }

    #[test]
    fn test_publisher() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );
        store.add_publisher(pub1.clone()).unwrap();

        let pub2 = store.publisher("topic1", "p_uuid1", "n_uuid1");
        assert_eq!(Some(&pub1), pub2);
    }

    #[test]
    fn test_publishers() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );
        store.add_publisher(pub1.clone()).unwrap();

        let pub2 = create_msg_publisher(
            "topic2",
            "addr2",
            "p_uuid2",
            "n_uuid2",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );
        store.add_publisher(pub2.clone()).unwrap();

        let mut publisher = store.publishers(Some("topic1"), Some("p_uuid1"), Some("n_uuid1"));
        assert_eq!(publisher.len(), 1);
        publisher = store.publishers(Some("topic1"), Some("p_uuid1"), None);
        assert_eq!(publisher.len(), 1);
        publisher = store.publishers(Some("topic1"), None, None);
        assert_eq!(publisher.len(), 1);
        publisher = store.publishers(Some("topic1"), None, Some("n_uuid1"));
        assert_eq!(publisher.len(), 1);
        publisher = store.publishers(None, Some("p_uuid1"), Some("n_uuid1"));
        assert_eq!(publisher.len(), 1);
        publisher = store.publishers(None, Some("p_uuid1"), None);
        assert_eq!(publisher.len(), 1);
        publisher = store.publishers(None, None, Some("n_uuid1"));
        assert_eq!(publisher.len(), 1);
        publisher = store.publishers(None, None, None);
        assert_eq!(publisher.len(), 2);
    }

    #[test]
    fn test_del_publisher_by_node() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub2 = create_msg_publisher(
            "topic2",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub3 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid2",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        store.add_publisher(pub2.clone()).unwrap();
        store.add_publisher(pub3.clone()).unwrap();

        // let pubs = store.publishers("topic1");
        // assert_eq!(pubs.len(), 2);

        let result = store.del_publisher_by_node("topic1", "p_uuid1", "n_uuid1");
        assert_eq!(result.is_ok(), true);

        let result = store.del_publisher_by_node("topic1", "p_uuid1", "n_uuid1");
        assert_eq!(result.is_err(), true);

        let pubs = store.publishers(None, None, None);
        assert_eq!(pubs.len(), 1);
    }

    #[test]
    fn test_del_publisher_by_process() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub2 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid2",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub3 = create_msg_publisher(
            "topic2",
            "addr1",
            "p_uuid2",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        store.add_publisher(pub2.clone()).unwrap();
        store.add_publisher(pub3.clone()).unwrap();

        let pubs = store.publishers(Some("topic1"), None, None);
        assert_eq!(pubs.len(), 2);

        let result = store.del_publishers_by_process("p_uuid1");
        assert_eq!(result.is_ok(), true);

        let result = store.del_publishers_by_process("p_uuid1");
        assert_eq!(result.is_err(), true);

        let pubs = store.publishers(Some("topic1"), None, None);
        assert_eq!(pubs.len(), 0);

        let pubs = store.publishers(Some("topic2"), None, None);
        assert_eq!(pubs.len(), 1);
    }

    #[test]
    fn test_publishers_by_process() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub2 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid2",
            "n_uuid2",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        store.add_publisher(pub2.clone()).unwrap();

        let pubs = store.publishers_by_process("p_uuid1");
        assert_eq!(pubs.len(), 1);

        let pubs = store.publishers_by_process("p_uuid2");
        assert_eq!(pubs.len(), 1);

        let pubs = store.publishers_by_process("p_uuid3");
        assert_eq!(pubs.len(), 0);
    }

    #[test]
    fn test_publishers_by_node() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub2 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid2",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        store.add_publisher(pub2.clone()).unwrap();

        let pubs = store.publishers_by_node("p_uuid1", "n_uuid1");
        assert_eq!(pubs.len(), 1);

        let pubs = store.publishers_by_node("p_uuid2", "n_uuid2");
        assert_eq!(pubs.len(), 0);

        let pubs = store.publishers_by_node("p_uuid1", "n_uuid2");
        assert_eq!(pubs.len(), 1);
    }

    #[test]
    fn test_topic_list() {
        let mut store = DiscoveryStore::new();
        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub2 = create_msg_publisher(
            "topic2",
            "addr1",
            "p_uuid1",
            "n_uuid2",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        store.add_publisher(pub2.clone()).unwrap();

        let topics = store.topic_list();
        assert_eq!(topics.len(), 2);
    }

    #[test]
    fn test_print() {
        let store = DiscoveryStore::new();

        let pub1 = create_msg_publisher(
            "topic1",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let mut store = DiscoveryStore::new();
        let pub2 = create_msg_publisher(
            "topic2",
            "addr2",
            "p_uuid2",
            "n_uuid2",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        let pub3 = create_msg_publisher(
            "topic3",
            "addr1",
            "p_uuid1",
            "n_uuid1",
            DiscoveryScope::All,
            "gz.msgs.StringMsg",
        );

        store.add_publisher(pub1.clone()).unwrap();
        store.add_publisher(pub3.clone()).unwrap();
        store.add_publisher(pub2.clone()).unwrap();

        store.print();
    }
}
