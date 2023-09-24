use std::fmt::Debug;
use anyhow::Result;

mod service;
mod request;
mod subscription;
mod store;
mod response;


pub(crate) use store::{DispatcherStore, DeleteFunction, CleanFunction};
pub(crate) use subscription::{Subscriber};
pub(crate) use request::{PendingRequest};
pub(crate) use response::{ResponseDispatcher};
pub(crate) use service::{ServiceDispatcher};

pub(crate) trait Dispatcher: Debug {
    fn uuid(&self) -> &str;
    fn topic(&self) -> &str;
    fn node_uuid(&self) -> &str;
    fn request_type(&self) -> Option<&str>;
    fn response_type(&self) -> Option<&str>;
}