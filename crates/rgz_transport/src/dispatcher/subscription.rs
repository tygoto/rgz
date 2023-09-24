use std::fmt::Debug;
use anyhow::{bail, Result};
use tokio::sync::mpsc::UnboundedSender;

use crate::dispatcher::Dispatcher;
use crate::transport::PublishMessage;

pub(crate) struct Subscriber {
    h_uuid: String,
    p_uuid: String,
    n_uuid: String,
    topic: String,
    msg_type: String,
    sender: Option<UnboundedSender<PublishMessage>>,
    remote_flag: bool,
}

impl Subscriber {
    pub(crate) fn new(
        process_uuid: &str,
        node_uuid: &str,
        topic: &str,
        msg_type: &str,
        sender: Option<UnboundedSender<PublishMessage>>,
    ) -> Self {
        let remote_flag = sender.is_none();
        Subscriber {
            h_uuid: uuid::Uuid::new_v4().to_string(),
            p_uuid: process_uuid.to_string(),
            n_uuid: node_uuid.to_string(),
            topic: topic.to_string(),
            msg_type: msg_type.to_string(),
            sender,
            remote_flag,
        }
    }

    pub fn dispatch(&mut self, publish_message: PublishMessage) -> Result<()> {
        if let Some(sender) = self.sender.as_ref(){
            sender.send(publish_message)?;
            Ok(())
        }else {
            bail!("Sender not found");
        }
    }

    pub fn is_remote(&self) -> bool {
        self.remote_flag
    }

    pub(crate) fn process_uuid(&self) -> &str {
        self.p_uuid.as_str()
    }

}

impl Dispatcher for Subscriber {
    fn uuid(&self) -> &str {
        self.h_uuid.as_str()
    }
    fn topic(&self) -> &str {
        self.topic.as_str()
    }
    fn node_uuid(&self) -> &str {
        self.n_uuid.as_str()
    }
    fn request_type(&self) -> Option<&str> {
        Some(self.msg_type.as_str())
    }
    fn response_type(&self) -> Option<&str> {
        None
    }
}

impl Debug for Subscriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscriber")
            .field("h_uuid", &self.h_uuid)
            .field("p_uuid", &self.p_uuid)
            .field("n_uuid", &self.n_uuid)
            .field("topic", &self.topic)
            .field("msg_type", &self.msg_type)
            .finish()
    }
}
