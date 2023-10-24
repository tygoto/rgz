use crate::dispatcher::Dispatcher;
use crate::transport::{ReplyMessage, RequestMessage};
use std::fmt::Debug;
use tokio::sync::oneshot;

pub(crate) struct PendingRequest {
    req_uuid: String,
    n_uuid: String,
    topic: String,
    req_type: Option<String>,
    res_type: Option<String>,
    request_message: Option<RequestMessage>,
    sender: Option<oneshot::Sender<ReplyMessage>>,
    created: std::time::Instant,
}

impl PendingRequest {
    pub fn new(request_message: RequestMessage, sender: oneshot::Sender<ReplyMessage>) -> Self {
        PendingRequest {
            req_uuid: request_message.req_uuid.clone(),
            n_uuid: request_message.node_uuid.clone(),
            topic: request_message.topic.clone(),
            req_type: Some(request_message.req_type.clone()),
            res_type: Some(request_message.res_type.clone()),
            request_message: Some(request_message),
            sender: Some(sender),
            created: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.created.elapsed()
    }

    pub fn requested(&self) -> bool {
        self.request_message.is_none()
    }

    pub fn message(&mut self) -> Option<RequestMessage> {
        self.request_message.take()
    }
    pub fn sender(&mut self) -> Option<oneshot::Sender<ReplyMessage>> {
        self.sender.take()
    }
}

impl Debug for PendingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingRequest")
            .field("req_uuid", &self.req_uuid)
            .field("topic", &self.topic)
            .field("n_uuid", &self.n_uuid)
            .field("req_type", &self.req_type)
            .field("res_type", &self.res_type)
            .finish()
    }
}

impl Dispatcher for PendingRequest {
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
