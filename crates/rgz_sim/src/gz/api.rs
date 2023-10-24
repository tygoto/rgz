use anyhow::Result;
use std::ops::Deref;

use bevy::prelude::*;
use bevy::utils::HashMap;
use prost::Message;
use rgz_msgs::GzMessage;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::gz::event::{PoseEvent, SceneEvent};
use crate::gz::service::{GzService, ServiceResponse, ServiceTopic, Subscription};
use rgz_msgs as msgs;

#[derive(Resource, Deref, DerefMut)]
pub struct AsyncRuntime(pub(crate) Runtime);

#[derive(Resource)]
pub struct GzAPI {
    world_name: String,
    service: GzService,
}
impl GzAPI {
    fn new(service: GzService) -> Self {
        Self {
            world_name: "empty".to_string(),
            service,
        }
    }

    /// init the world
    pub(super) fn init(&mut self) {
        self.service.init();
    }

    pub(super) fn recv_response(&mut self, mut scene_event: EventWriter<SceneEvent>) {
        while let Some(response) = self.service.try_recv_response() {
            match response {
                ServiceResponse::SceneInfo { world_name, result } => {
                    if world_name == self.world_name {
                        if let Ok(scene) = result {
                            scene_event.send(SceneEvent::Renew(scene));
                        }
                    }
                }
            }
        }
    }

    pub(super) fn recv_subscription(
        &mut self,
        mut pose_event: EventWriter<PoseEvent>,
        mut scene_event: EventWriter<SceneEvent>,
    ) {
        while let Some(subscription) = self.service.try_recv_subscription() {
            match subscription {
                Subscription::GuiCameraPose(_) => {}
                Subscription::GazeboResourcePaths(_) => {}
                Subscription::Clock { .. } => {}
                Subscription::DynamicPoseInfo { world_name, pose_v } => {
                    if world_name == self.world_name {
                        let mut pose_map = HashMap::<u32, msgs::Pose>::new();
                        for pose in pose_v.pose {
                            pose_map.insert(pose.id, pose);
                        }
                        pose_event.send(PoseEvent { pose_map });
                    }
                }
                Subscription::PoseInfo { .. } => {}
                Subscription::SceneDeletion { .. } => {}
                Subscription::SceneInfo { .. } => {}
                Subscription::State { .. } => {}
                Subscription::Stats { .. } => {}
                Subscription::LightConfig { .. } => {}
                Subscription::Unknown => {}
            }
        }
    }

    pub fn set_world_name(&mut self, world_name: &str) {
        self.world_name = world_name.to_string();

        self.service
            .subscribe(ServiceTopic::DynamicPoseInfo(self.world_name.clone()));
    }
    pub fn init_scene(&mut self) {
        self.service.scene_info(&self.world_name);
    }
}

impl FromWorld for GzAPI {
    fn from_world(world: &mut World) -> Self {
        if world.get_resource::<AsyncRuntime>().is_none() {
            let async_runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            world.insert_resource(AsyncRuntime(async_runtime));
        };
        let runtime = world.resource::<AsyncRuntime>();
        let service = GzService::new(runtime.handle().clone());
        GzAPI::new(service)
    }
}
