use anyhow::Result;
use bevy::log::error;
use bevy::prelude::info;
use bevy::utils::HashMap;
use regex::Regex;
use rgz_msgs as msgs;
use rgz_transport::Node;
use tokio::runtime;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub(crate) enum ServiceTopic {
    GazeboResourcePaths,
    GuiCameraPose,
    Clock(Option<String>),
    DynamicPoseInfo(String),
    PoseInfo(String),
    SceneDeletion(String),
    SceneInfo(String),
    State(String),
    Stats(Option<String>),
    LightConfig(String),
    Unknown,
}

impl ServiceTopic {
    pub fn name(&self) -> String {
        match self {
            Self::GazeboResourcePaths => "/gazebo/resource_paths".to_string(),
            Self::GuiCameraPose => "/gui/camera/pose".to_string(),
            Self::Clock(name) => match name {
                Some(name) => format!("/world/{}/clock", name),
                None => "/clock".to_string(),
            },
            Self::DynamicPoseInfo(name) => format!("/world/{}/dynamic_pose/info", name),
            Self::PoseInfo(name) => format!("/world/{}/pose/info", name),
            Self::SceneDeletion(name) => format!("/world/{}/scene/deletion", name),
            Self::SceneInfo(name) => format!("/world/{}/scene/info", name),
            Self::State(name) => format!("/world/{}/state", name),
            Self::Stats(name) => match name {
                Some(name) => format!("/world/{}/stats", name),
                None => "/stats".to_string(),
            },
            Self::LightConfig(name) => format!("/world/{}/light_config", name),
            _ => "unknown".to_string(),
        }
    }
    pub fn from_str_name(value: &str) -> Option<Self> {
        let re = Regex::new(r"^/world/([^/]+)/(.+)$").unwrap();

        if let Some(caps) = re.captures(value) {
            let world_name = caps.get(1).unwrap().as_str().to_string();
            let topic = caps.get(2).unwrap().as_str().to_string();

            match topic.as_str() {
                "clock" => Some(Self::Clock(Some(world_name))),
                "dynamic_pose/info" => Some(Self::DynamicPoseInfo(world_name)),
                "pose/info" => Some(Self::PoseInfo(world_name)),
                "scene/deletion" => Some(Self::SceneDeletion(world_name)),
                "scene/info" => Some(Self::SceneInfo(world_name)),
                "state" => Some(Self::State(world_name)),
                "stats" => Some(Self::Stats(Some(world_name))),
                "light_config" => Some(Self::LightConfig(world_name)),
                _ => None,
            }
        } else {
            match value {
                "/clock" => Some(Self::Clock(None)),
                "/gazebo/resource_paths" => Some(Self::GazeboResourcePaths),
                "/gui/camera/pose" => Some(Self::GuiCameraPose),
                "/stats" => Some(Self::Stats(None)),
                _ => None,
            }
        }
    }
}

pub(crate) enum Subscription {
    GuiCameraPose(msgs::Pose),
    GazeboResourcePaths(msgs::StringMsgV),
    Clock {
        world_name: Option<String>,
        clock: msgs::Clock,
    },
    DynamicPoseInfo {
        world_name: String,
        pose_v: msgs::PoseV,
    },
    PoseInfo {
        world_name: String,
        pose_v: msgs::PoseV,
    },
    SceneDeletion {
        world_name: String,
        gz_id: msgs::UInt32V,
    },
    SceneInfo {
        world_name: String,
        scene: msgs::Scene,
    },
    State {
        world_name: String,
        state: msgs::SerializedStepMap,
    },
    Stats {
        world_name: Option<String>,
        stats: msgs::WorldStatistics,
    },
    LightConfig {
        world_name: String,
        light: msgs::Light,
    },
    Unknown,
}

enum ServiceRequest {
    Subscribe(ServiceTopic),
    Unsubscribe(ServiceTopic),
    SceneInfo(String),
}
pub(crate) enum ServiceResponse {
    SceneInfo {
        world_name: String,
        result: Result<msgs::Scene>,
    },
}

pub(crate) struct GzService {
    runtime: runtime::Handle,

    request_sender: Option<Sender<ServiceRequest>>,
    response_receiver: Receiver<ServiceResponse>,
    response_sender: Sender<ServiceResponse>,
    subscription_receiver: Receiver<Subscription>,
    subscription_sender: Sender<Subscription>,
}

impl GzService {
    pub(super) fn new(runtime_handle: runtime::Handle) -> Self {
        let (response_sender, response_receiver) = mpsc::channel(100);
        let (subscription_sender, subscription_receiver) = mpsc::channel(100);

        Self {
            runtime: runtime_handle,
            request_sender: None,
            response_receiver,
            response_sender,
            subscription_receiver,
            subscription_sender,
        }
    }

    pub(super) fn init(&mut self) {
        let (request_sender, request_receiver) = mpsc::channel(100);

        self.request_sender = Some(request_sender);
        let response_sender = self.response_sender.clone();
        let subscription_sender = self.subscription_sender.clone();

        self.runtime.spawn(async move {
            let node = Node::new(None);
            service_task(node, request_receiver, response_sender, subscription_sender).await;
        });
    }
    pub(super) fn try_recv_subscription(&mut self) -> Option<Subscription> {
        self.subscription_receiver.try_recv().ok()
    }
    pub(super) fn try_recv_response(&mut self) -> Option<ServiceResponse> {
        self.response_receiver.try_recv().ok()
    }

    pub fn subscribe(&self, topic: ServiceTopic) {
        if let Some(request_sender) = &self.request_sender {
            let _ = request_sender.blocking_send(ServiceRequest::Subscribe(topic));
        }
    }

    pub fn scene_info(&self, world_name: &str) {
        if let Some(request_sender) = &self.request_sender {
            let _ = request_sender.blocking_send(ServiceRequest::SceneInfo(world_name.to_string()));
        }
    }
}
async fn service_task(
    mut node: Node,
    mut request_receiver: Receiver<ServiceRequest>,
    response_sender: Sender<ServiceResponse>,
    subscription_sender: Sender<Subscription>,
) {
    loop {
        select! {
            Some(request) = request_receiver.recv() => {
                match request {
                    ServiceRequest::Subscribe(topic) => {
                        let topic_name = topic.name();

                        match topic {
                            ServiceTopic::GazeboResourcePaths => {

                            }
                            ServiceTopic::GuiCameraPose => {}
                            ServiceTopic::Clock(_) => {}
                            ServiceTopic::DynamicPoseInfo(world_name) => {
                                let n = world_name.clone();
                                let s = subscription_sender.clone();
                                let _ = node.subscribe(&topic_name,
                                    move |pose_v: msgs::PoseV| {
                                        let name = n.clone();
                                        let sender = s.clone();
                                        async move {
                                            sender.send(Subscription::DynamicPoseInfo {
                                                world_name: name,
                                                pose_v,
                                            }).await?;
                                            Ok(())
                                        }
                                    }
                                );
                            }
                            ServiceTopic::PoseInfo(_) => {
                                let n = topic.name();
                                let s = subscription_sender.clone();
                                let _ = node.subscribe(&topic.name(),
                                    move |pose_v: msgs::PoseV| {
                                        let name = n.clone();
                                        let sender = s.clone();
                                        async move {
                                            sender.send(Subscription::PoseInfo {
                                                world_name: name,
                                                pose_v,
                                            }).await?;
                                            Ok(())
                                        }
                                    }
                                );
                            }
                            ServiceTopic::SceneDeletion(_) => {}
                            ServiceTopic::SceneInfo(_) => {}
                            ServiceTopic::State(_) => {}
                            ServiceTopic::Stats(_) => {}
                            ServiceTopic::LightConfig(_) => {}
                            ServiceTopic::Unknown => {}
                        }
                    }

                    ServiceRequest::Unsubscribe(topic) => {
                        let name = topic.name();
                    }

                    ServiceRequest::SceneInfo(name) => {
                        let service_name = format!("/world/{}/scene/info", name.as_str());
                        let result = node.request::<msgs::Empty, msgs::Scene>(&service_name, None, None).await;
                        if let Err(_) = response_sender.send(ServiceResponse::SceneInfo {
                            world_name: name,
                            result,
                        }).await {
                            error!("failed to send response for scene info");
                        }
                    }
                }
            },
        }
    }
}
