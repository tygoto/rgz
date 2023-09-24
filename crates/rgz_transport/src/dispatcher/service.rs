use anyhow::Result;
use std::fmt::{Debug};
use tokio::sync::mpsc::UnboundedSender;

use crate::dispatcher::Dispatcher;
use crate::transport::{RequestMessage};

pub(crate) struct ServiceDispatcher {
    s_uuid: String,
    topic: String,
    n_uuid: String,
    req_type: Option<String>,
    res_type: Option<String>,

    request_sender: UnboundedSender<RequestMessage>,
}

impl ServiceDispatcher {
    pub fn new(
        node_uuid: &str,
        topic: &str,
        request_type: Option<&str>,
        response_type: Option<&str>,
        request_sender: UnboundedSender<RequestMessage>,
    ) -> Self {
        ServiceDispatcher {
            s_uuid: uuid::Uuid::new_v4().to_string(),
            topic: topic.to_string(),
            n_uuid: node_uuid.to_string(),
            req_type: request_type.map(|s| s.to_string()),
            res_type: response_type.map(|s| s.to_string()),
            request_sender,
        }
    }

    pub fn request(&mut self, msg: RequestMessage) -> Result<()> {
        self.request_sender.send(msg)?;
        Ok(())
    }
}

impl Debug for ServiceDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceDispatcher")
            .field("s_uuid", &self.s_uuid)
            .field("n_uuid", &self.n_uuid)
            .field("topic", &self.topic)
            .field("req_type", &self.req_type)
            .field("res_type", &self.res_type)
            .finish()
    }
}

impl Dispatcher for ServiceDispatcher {
    fn uuid(&self) -> &str {
        self.s_uuid.as_str()
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