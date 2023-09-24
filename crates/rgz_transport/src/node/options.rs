use std::collections::HashMap;
use anyhow::{Result, bail};

use crate::discovery::DiscoveryScope;
use crate::utils::topic as topic_utils;
use crate::utils::net as net_utils;

const UNTHROTTLED: u64 = u64::MAX;

#[derive(Debug, Clone)]
pub struct AdvertiseOptions {
    scope: DiscoveryScope,
    msgs_per_sec: u64
}
impl AdvertiseOptions {
    pub fn new() -> Self {
        Self {
            scope: DiscoveryScope::All,
            msgs_per_sec: UNTHROTTLED,
        }
    }

    pub fn scope(&self) -> DiscoveryScope {
        self.scope
    }
    pub fn set_scope(&mut self, scope: DiscoveryScope) {
        self.scope = scope;
    }

    pub fn msgs_per_sec(&self) -> u64 {
        self.msgs_per_sec
    }

    pub fn set_msgs_per_sec(&mut self, msgs_per_sec: u64) {
        self.msgs_per_sec = msgs_per_sec;
    }

    pub fn throttled(&self) -> bool {
        self.msgs_per_sec != UNTHROTTLED
    }
}

impl Default for AdvertiseOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeOptions {
    ns: String,
    partition: String,
    topics_remap: HashMap<String, String>,
}
impl NodeOptions {
    pub fn new() -> Self {
        Self {
            ns: "".to_string(),
            partition: format!("{}:{}", net_utils::hostname(), net_utils::username()),
            topics_remap: HashMap::new(),
        }
    }

    pub fn namespace(&self) -> &String {
        &self.ns
    }
    pub fn set_namespace(&mut self, name_space: &str) {
        self.ns = name_space.to_string();
    }

    /// Get the partition used in this node.
    pub fn partition(&self) -> &String {
        &self.partition
    }

    pub fn set_partition(&mut self, partition: &str){
        self.partition = partition.to_string();
    }

    pub fn add_topic_remap(&mut self, from_topic: &str, to_topic: &str) -> Result<()> {
        // Sanity check: Make sure that both topics are valid.
        for topic in &[from_topic, to_topic] {
            if !topic_utils::is_valid_topic(topic) {
                bail!("Invalid topic name [{}]", topic);
            }
        }

        // Sanity check: Make sure that the original topic hasn't been remapped already
        if let Some(remapped_topic) = self.topics_remap.get(from_topic) {
            bail!(
                "Topic name [{}] has already been remapped to [{}]",
                from_topic, remapped_topic
            );
        }

        self.topics_remap.insert(from_topic.to_string(), to_topic.to_string());

        Ok(())
    }

    pub fn topic_remap(&self, from_topic: &str) -> Option<&String> {
        // Is there any remap for this topic?
        self.topics_remap.get(from_topic)
    }

}

impl Default for NodeOptions {
    fn default() -> Self {
        Self::new()
    }
}


