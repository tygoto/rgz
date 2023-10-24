use anyhow::{bail, Result};
use std::fmt::Debug;
use tokio::sync::oneshot;

use crate::dispatcher::Dispatcher;
use crate::transport::{ReplyMessage, RequestMessage};

pub(crate) struct ResponseDispatcher {
    req_uuid: String,
    n_uuid: String,
    topic: String,
    req_type: Option<String>,
    res_type: Option<String>,
    sender: Option<oneshot::Sender<ReplyMessage>>,
    remote_flag: bool,
    created: std::time::Instant,
    done_flag: bool,
}

impl ResponseDispatcher {
    pub fn new(
        request_message: &RequestMessage,
        sender: Option<oneshot::Sender<ReplyMessage>>,
    ) -> Self {
        let remote_flag = sender.is_none();
        ResponseDispatcher {
            req_uuid: request_message.req_uuid.clone(),
            n_uuid: request_message.node_uuid.clone(),
            topic: request_message.topic.clone(),
            req_type: Some(request_message.req_type.clone()),
            res_type: Some(request_message.res_type.clone()),
            sender,
            remote_flag,
            created: std::time::Instant::now(),
            done_flag: false,
        }
    }

    pub fn dispatch(&mut self, reply_message: ReplyMessage) -> Result<()> {
        if let Some(sender) = self.sender.take() {
            if let Err(err) = sender.send(reply_message) {
                bail!("Failed to send: {:?}", err)
            }
            Ok(())
        } else {
            bail!("Sender not found");
        }
    }

    pub fn is_remote(&self) -> bool {
        self.remote_flag
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.created.elapsed()
    }

    pub fn done(&mut self) {
        self.done_flag = true;
    }

    pub fn is_done(&self) -> bool {
        self.done_flag
    }
}

impl Debug for ResponseDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseHandler")
            .field("req_uuid", &self.req_uuid)
            .field("topic", &self.topic)
            .field("n_uuid", &self.n_uuid)
            .field("req_type", &self.req_type)
            .field("res_type", &self.res_type)
            .finish()
    }
}

impl Dispatcher for ResponseDispatcher {
    fn uuid(&self) -> &str {
        self.req_uuid.as_str()
    }

    fn topic(&self) -> &str {
        self.topic.as_str()
    }

    fn node_uuid(&self) -> &str {
        self.n_uuid.as_str()
    }

    fn request_type(&self) -> Option<&str> {
        self.req_type.as_deref()
    }

    fn response_type(&self) -> Option<&str> {
        self.res_type.as_deref()
    }
}
