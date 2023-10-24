use anyhow::Result;
use std::fmt::Debug;

mod request;
mod response;
mod service;
mod store;
mod subscription;

pub(crate) use request::PendingRequest;
pub(crate) use response::ResponseDispatcher;
pub(crate) use service::ServiceDispatcher;
pub(crate) use store::{CleanFunction, DeleteFunction, DispatcherStore};
pub(crate) use subscription::Subscriber;

pub(crate) trait Dispatcher: Debug {
    fn uuid(&self) -> &str;
    fn topic(&self) -> &str;
    fn node_uuid(&self) -> &str;
    fn request_type(&self) -> Option<&str>;
    fn response_type(&self) -> Option<&str>;
}
