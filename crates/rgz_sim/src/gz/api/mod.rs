mod topic;
mod service;

pub(crate) use topic::{GzTopic};
pub(crate) use service::{GzService};

use bevy::prelude::*;
use tokio::{runtime, select};
use tokio::sync::mpsc::{self, Receiver, Sender};
use rgz_transport::Node as TransportNode;
use rgz_msgs as msgs;


#[derive(Resource, Deref, DerefMut)]
pub struct AsyncRuntime(pub(crate) runtime::Runtime);

#[derive(Resource)]
pub struct GzAPI {
    world_name: String,
    runtime_handle: runtime::Handle,
    subscription_receiver: Receiver<GzTopic>,
    response_receiver: Receiver<GzService>,
    pub_req_sender: Sender<GzTopic>,
    sub_req_sender: Sender<GzTopic>,
    unsub_req_sender: Sender<GzTopic>,
    srv_req_sender: Sender<GzService>,
}

impl GzAPI {
    fn new(runtime_handle: runtime::Handle) -> Self {

        let (subscription_sender, subscription_receiver) = mpsc::channel::<GzTopic>(100);
        let (response_sender, response_receiver) = mpsc::channel::<GzService>(100);
        let (pub_req_sender, pub_req_receiver) = mpsc::channel::<GzTopic>(100);
        let (sub_req_sender, sub_req_receiver) = mpsc::channel::<GzTopic>(100);
        let (unsub_req_sender, unsub_req_receiver) = mpsc::channel::<GzTopic>(100);
        let (srv_req_sender, srv_req_receiver) = mpsc::channel::<GzService>(100);

        runtime_handle.spawn(async move {
           api_task(
                pub_req_receiver,
                sub_req_receiver,
                unsub_req_receiver,
                srv_req_receiver,
                subscription_sender,
                response_sender,
            ).await;
        });

        Self {
            world_name: "empty".to_string(),
            runtime_handle,
            subscription_receiver,
            response_receiver,
            pub_req_sender,
            sub_req_sender,
            unsub_req_sender,
            srv_req_sender,
        }
    }

    pub(super) fn try_recv_subscription(&mut self) -> Option<GzTopic> {
        self.subscription_receiver.try_recv().ok()
    }
    pub(super) fn try_recv_response(&mut self) -> Option<GzService> {
        self.response_receiver.try_recv().ok()
    }

    pub fn world_name(&self) -> &str {
        &self.world_name
    }
    pub fn set_world(&mut self, name: &str) {
        self.world_name = name.to_string();

        let _ = self.srv_req_sender.blocking_send(GzService::WorldSceneInfo {
            world_name: self.world_name.clone(),
            request: msgs::Empty {
                ..Default::default()
            },
            response: None,
        });

        let _ = self.sub_req_sender.blocking_send(
        GzTopic::WorldDynamicPoseInfo(self.world_name.clone(), None)
        );
    }

    pub fn play(&mut self) {
        let _ = self.srv_req_sender.blocking_send(GzService::WorldControl {
            world_name: self.world_name.clone(),
            request: msgs::WorldControl {
                pause: false,
                ..Default::default()
            },
            response: None,
        });
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
        GzAPI::new(runtime.handle().clone())
    }
}


async fn api_task(
    mut pub_req_receiver: Receiver<GzTopic>,
    mut sub_req_receiver: Receiver<GzTopic>,
    mut unsub_req_receiver: Receiver<GzTopic>,
    mut srv_req_receiver: Receiver<GzService>,
    subscription_sender: Sender<GzTopic>,
    response_sender: Sender<GzService>,
)
{
    let mut node = TransportNode::new(None);

    loop {
        select! {
            Some(topic) = pub_req_receiver.recv() => {
                let topic_name = topic.name();
                match topic {
                    GzTopic::Clock(clock) => {}
                    GzTopic::GazeboResourcePaths(paths) => {
                    }
                    GzTopic::GuiCameraPose(pose) => {}
                    GzTopic::Stats(stats) => {}
                    GzTopic::WorldClock(_, clock) => {}
                    GzTopic::WorldDynamicPoseInfo(_, pose_v) => {

                    }
                    GzTopic::WorldPoseInfo(_, pose_v) => {}
                    GzTopic::WorldSceneDeletion(_, ids) => {}
                    GzTopic::WorldSceneInfo(_, scene) => {}
                    GzTopic::WorldState(_, state) => {}
                    GzTopic::WorldStats(_, stats) => {}
                    _ => {}
                }
            },

            Some(topic) = sub_req_receiver.recv() => {
                let topic_name = topic.name();
                match topic {
                    GzTopic::Clock(..) => {}
                    GzTopic::GazeboResourcePaths(..) => {
                    }
                    GzTopic::GuiCameraPose(..) => {}
                    GzTopic::Stats(..) => {}
                    GzTopic::WorldClock(..) => {}
                    GzTopic::WorldDynamicPoseInfo(name, ..) => {
                        let world_name = name.clone();
                        let mut sender = subscription_sender.clone();
                        if let Err(e) = node.subscribe(&topic_name,
                            move |pose_v: msgs::PoseV| {

                                if let Err(e) = sender.try_send(
                                    GzTopic::WorldDynamicPoseInfo(world_name.to_string(), Some(pose_v))
                                ) {
                                    error!("failed to send subscription: {}", e);
                                };

                            }
                        ){
                            error!("failed to subscribe to topic: {}", topic_name);
                        }
                    }
                    GzTopic::WorldPoseInfo(..) => {}
                    GzTopic::WorldSceneDeletion(..) => {}
                    GzTopic::WorldSceneInfo(..) => {}
                    GzTopic::WorldState(..) => {}
                    GzTopic::WorldStats(..) => {}
                    _ => {}
                }
            },

            Some(topic) = unsub_req_receiver.recv() => {
                let topic_name = topic.name();
            },

            Some(service) = srv_req_receiver.recv() => {
                let service_name = service.name();

                match service {
                    GzService::GazeboResourcePathsAdd { request, .. } => {}
                    GzService::GazeboResourcePathsGet { request, .. } => {}
                    GzService::GazeboResourcePathsResolve { request, .. } => {}
                    GzService::GazeboWorlds { request, .. } => {}
                    GzService::GuiCameraViewControl { request, .. } => {}
                    GzService::GuiCameraViewControlReferenceVisual { request, .. } => {}
                    GzService::GuiCameraViewControlSensitivity { request, .. } => {}
                    GzService::GuiCopy { request, .. } => {}
                    GzService::GuiFollow { request, .. } => {}
                    GzService::GuiFollowOffset { request, .. } => {}
                    GzService::GuiMoveTo { request, .. } => {}
                    GzService::GuiMoveToPose { request, .. } => {}
                    GzService::GuiPaste { request, .. } => {}
                    GzService::GuiScreenshot { request, .. } => {}
                    GzService::GuiViewCollisions { request, .. } => {}
                    GzService::GuiViewCom { request, .. } => {}
                    GzService::GuiViewFrames { request, .. } => {}
                    GzService::GuiViewInertia { request, .. } => {}
                    GzService::GuiViewJoints { request, .. } => {}
                    GzService::GuiViewTransparent { request, .. } => {}
                    GzService::GuiViewWireframes { request, .. } => {}
                    GzService::Marker { request, .. } => {}
                    GzService::MarkerList { request, .. } => {}
                    GzService::MarkerArray { request, .. } => {}
                    GzService::ServerControl { request, .. } => {

                    }
                    GzService::WorldControl { world_name, request, .. } => {
                        if let Ok(res) = node.request::<msgs::WorldControl, msgs::Boolean>(&service_name, Some(request.clone()), None).await {
                            if let Err(_) = response_sender.try_send(GzService::WorldControl {
                                world_name: world_name.clone(),
                                request: request.clone(),
                                response: res,
                            }) {
                                error!("failed to send response for scene info");
                            }
                        }
                    }
                    GzService::WorldControlState { request, .. } => {

                    }
                    GzService::WorldCreate { request, .. } => {

                    }
                    GzService::WorldCreateMultiple { request, .. } => {

                    }
                    GzService::WorldDeclareParameter { request, .. } => {

                    }
                    GzService::WorldDisableCollision { request, .. } => {

                    }
                    GzService::WorldEnableCollision { request, .. } => {

                    }
                    GzService::WorldEntitySystemAdd { request, .. } => {

                    }
                    GzService::WorldGenerateWorldSdf { request, .. } => {

                    }
                    GzService::WorldGetParameter { request, .. } => {

                    }
                    GzService::WorldGuiInfo { request, .. } => {

                    }
                    GzService::WorldLevelSetPerformer { request, .. } => {

                    }
                    GzService::WorldLightConfig { request, .. } => {

                    }
                    GzService::WorldListParameters { request, .. } => {

                    }
                    GzService::WorldPlaybackControl { request, .. } => {

                    }
                    GzService::WorldRemove { request, .. } => {

                    }
                    GzService::WorldSceneGraph { request, .. } => {

                    }
                    GzService::WorldSceneInfo { world_name, request, .. } => {
                        if let Ok(scene) = node.request::<msgs::Empty, msgs::Scene>(&service_name, None, None).await {
                            if let Err(_) = response_sender.try_send(GzService::WorldSceneInfo {
                                world_name: world_name.clone(),
                                request: request.clone(),
                                response: scene,
                            }) {
                                error!("failed to send response for scene info");
                            }
                        }
                    }
                    GzService::WorldSetParameter { request, .. } => {

                    }
                    GzService::WorldSetPhysics { request, .. } => {

                    }
                    GzService::WorldSetPose { request, .. } => {

                    }
                    GzService::WorldSetPoseVector { request, .. } => {

                    }
                    GzService::WorldSetSphericalCoordinates { request, .. } => {

                    }
                    GzService::WorldState { request, .. } => {

                    }
                    GzService::WorldStateAsync { request, .. } => {

                    }
                    GzService::WorldSystemInfo { request, .. } => {

                    }
                    GzService::WorldVisualConfig { request, .. } => {

                    }
                    GzService::WorldWheelSlip { request, .. } => {

                    }
                    _ => {}
                }
            },
        }
    };
}

#[cfg(test)]
mod tests {
    use std::{env, thread};
    use std::thread::sleep;
    use bevy::utils::tracing;
    use super::*;

    fn runtime() -> runtime::Runtime {
        runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    }

    #[test]
    fn test_subscription_request() {

        env::set_var("GZ_IP", "172.17.0.1");

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        let (subscription_sender, mut subscription_receiver) = mpsc::channel::<GzTopic>(100);
        let (response_sender, mut response_receiver) = mpsc::channel::<GzService>(100);

        let (pub_req_sender, pub_req_receiver) = mpsc::channel::<GzTopic>(100);
        let (sub_req_sender, sub_req_receiver) = mpsc::channel::<GzTopic>(100);
        let (unsub_req_sender, unsub_req_receiver) = mpsc::channel::<GzTopic>(100);
        let (srv_req_sender, srv_req_receiver) = mpsc::channel::<GzService>(100);

        let binding = runtime();
        let handle = binding.handle();
        handle.spawn(async move {
            api_task(
                pub_req_receiver,
                sub_req_receiver,
                unsub_req_receiver,
                srv_req_receiver,
                subscription_sender,
                response_sender,
            ).await;
        });

        thread::spawn(move||{
            if let Err(e) = sub_req_sender.blocking_send(
                GzTopic::WorldDynamicPoseInfo("shapes".to_string(), None)
            ){
                error!("failed to send subscription request: {}", e);
            };

            loop {
                if let Some(topic) = subscription_receiver.try_recv().ok() {
                    match topic {
                        GzTopic::WorldDynamicPoseInfo(world_name, pose_v) => {
                            println!("pose_v: {:?}", pose_v);
                        }
                        _ => {}
                    }
                }else {
                    sleep(std::time::Duration::from_millis(100));
                }
            }
        });

        sleep(std::time::Duration::from_secs(1));
    }

}