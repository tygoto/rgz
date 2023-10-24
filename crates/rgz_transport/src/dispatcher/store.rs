use anyhow::{bail, Result};
use std::collections::HashMap;
use std::ops::Deref;
use std::time::Duration;

use crate::dispatcher::{Dispatcher, PendingRequest, ResponseDispatcher, Subscriber};

pub(crate) struct DispatcherStore<T> {
    // topic -> node_uuid -> dispatcher_uuid -> dispatcher
    topics: HashMap<String, HashMap<String, HashMap<String, T>>>,
}

impl<T> DispatcherStore<T>
where
    T: Dispatcher,
{
    pub fn new() -> Self {
        DispatcherStore {
            topics: HashMap::new(),
        }
    }

    pub fn register(&mut self, dispatcher: T) -> Result<()> {
        let topic = dispatcher.topic().to_string();
        let node_uuid = dispatcher.node_uuid().to_string();
        let dispatcher_uuid = dispatcher.uuid().to_string();

        let nodes = self.topics.entry(topic).or_insert(HashMap::new());
        if let Some(dispatchers) = nodes.get(&node_uuid) {
            if dispatchers.values().any(|d| d.uuid() == dispatcher.uuid()) {
                bail!("dispatcher already exists");
            }
        }

        nodes
            .entry(node_uuid)
            .or_insert(HashMap::new())
            .insert(dispatcher_uuid, dispatcher);

        Ok(())
    }
    pub fn remove(&mut self, topic: &str, node_uuid: &str, dispatcher_uuid: &str) -> Option<T> {
        let nodes = self.topics.get_mut(topic)?;
        let dispatchers = nodes.get_mut(node_uuid)?;
        let dispatcher = dispatchers.remove(dispatcher_uuid)?;

        if dispatchers.is_empty() {
            nodes.remove(node_uuid);
        }
        if nodes.is_empty() {
            self.topics.remove(topic);
        }
        Some(dispatcher)
    }

    pub fn remove_by_node(&mut self, topic: &str, node_uuid: &str) -> Option<Vec<T>> {
        if let Some(nodes) = self.topics.get_mut(topic) {
            let dispatchers = nodes.remove(node_uuid);
            if nodes.is_empty() {
                self.topics.remove(topic);
            }
            return dispatchers.map(|d| d.into_iter().map(|(_, d)| d).collect());
        }
        None
    }

    pub fn has_topic(&self, topic: &str) -> bool {
        self.topics.contains_key(topic)
    }

    pub fn find(
        &mut self,
        topic: &str,
        request_type: Option<&str>,
        response_type: Option<&str>,
    ) -> Option<&mut T> {
        self.topics.get_mut(topic).and_then(|nodes| {
            nodes
                .iter_mut()
                .flat_map(|(_, n)| n.values_mut())
                .find(|d| d.request_type() == request_type && d.response_type() == response_type)
        })
    }

    pub fn get(&mut self, topic: &str, node_uuid: &str, dispatcher_uuid: &str) -> Option<&mut T> {
        self.topics.get_mut(topic).and_then(|nodes| {
            nodes
                .get_mut(node_uuid)
                .and_then(|dispatchers| dispatchers.get_mut(dispatcher_uuid))
        })
    }

    pub fn get_for_topic(&mut self, topic: &str) -> Option<Vec<&mut T>> {
        self.topics.get_mut(topic).and_then(|nodes| {
            Some(
                nodes
                    .values_mut()
                    .flat_map(|dispatchers| dispatchers.values_mut())
                    .collect(),
            )
        })
    }

    pub fn filter(
        &mut self,
        topic: &str,
        request_type: Option<&str>,
        response_type: Option<&str>,
    ) -> Option<Vec<&mut T>> {
        self.topics.get_mut(topic).and_then(|nodes| {
            Some(
                nodes
                    .iter_mut()
                    .flat_map(|(_, n)| n.values_mut())
                    .filter(|d| {
                        d.request_type() == request_type && d.response_type() == response_type
                    })
                    .collect(),
            )
        })
    }

    pub(crate) fn print(&self) {
        println!("DispatcherStore:");
        self.topics.iter().for_each(|(topic, nodes)| {
            println!("  topic: {}", topic);
            nodes.iter().for_each(|(node_uuid, dispatchers)| {
                println!("    node_uuid: {}", node_uuid);
                dispatchers
                    .iter()
                    .for_each(|(dispatcher_uuid, dispatcher)| {
                        println!("      dispatcher_uuid: {}", dispatcher_uuid);
                        println!("        {:?}", dispatcher);
                    });
            });
        });
    }
}

pub(crate) trait DeleteFunction {
    fn del_by_process(&mut self, process_uuid: &str);
}

impl DeleteFunction for DispatcherStore<Subscriber> {
    fn del_by_process(&mut self, process_uuid: &str) {
        self.topics.iter_mut().for_each(|(_, nodes)| {
            nodes.iter_mut().for_each(|(_, dispatchers)| {
                dispatchers.retain(|_, d| d.process_uuid() != process_uuid);
            });
            nodes.retain(|_, dispatchers| !dispatchers.is_empty());
        });
        self.topics.retain(|_, nodes| !nodes.is_empty());
    }
}

pub(crate) trait CleanFunction {
    fn clean(&mut self, timeout: Option<Duration>);
}

impl CleanFunction for DispatcherStore<ResponseDispatcher> {
    fn clean(&mut self, timeout: Option<Duration>) {
        let timeout = timeout.unwrap_or(Duration::from_millis(5000));
        self.topics.iter_mut().for_each(|(_, nodes)| {
            nodes.iter_mut().for_each(|(_, dispatchers)| {
                dispatchers.retain(|_, d| !d.is_done() || d.elapsed() < timeout);
            });
            nodes.retain(|_, dispatchers| !dispatchers.is_empty());
        });
        self.topics.retain(|_, nodes| !nodes.is_empty());
    }
}

impl CleanFunction for DispatcherStore<PendingRequest> {
    fn clean(&mut self, timeout: Option<Duration>) {
        let timeout = timeout.unwrap_or(Duration::from_millis(5000));
        self.topics.iter_mut().for_each(|(_, nodes)| {
            nodes.iter_mut().for_each(|(_, dispatchers)| {
                dispatchers.retain(|_, d| !d.requested() || d.elapsed() < timeout);
            });
            nodes.retain(|_, dispatchers| !dispatchers.is_empty());
        });
        self.topics.retain(|_, nodes| !nodes.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_handler() {}
}
