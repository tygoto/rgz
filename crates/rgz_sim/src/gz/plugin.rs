use bevy::prelude::*;
use bevy::utils::HashMap;
use rgz_msgs as msgs;

use crate::AppArgs;
use crate::gz::api::{
    GzAPI,
    GzService,
    GzTopic,
};
use crate::gz::component::{GzId, GzScene};
use crate::gz::event::{PoseEvent, SceneEvent};
use crate::gz::spawn::spawn_scene;
use crate::gz::utils;

pub struct GzPlugin;
impl Default for GzPlugin {
    fn default() -> Self {
        GzPlugin {}
    }
}
impl Plugin for GzPlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        app.init_resource::<GzAPI>();
        app.add_event::<SceneEvent>();
        app.add_event::<PoseEvent>();
        // app.add_systems(PreStartup, pre_startup_system);
        app.add_systems(Update, event_trigger);
        app.add_systems(Update, (update_scene, update_pose));
    }
}
fn pre_startup_system(mut api: ResMut<GzAPI>, app_args: Res<AppArgs>) {

}
fn event_trigger(
    mut pose_event: EventWriter<PoseEvent>,
    mut scene_event: EventWriter<SceneEvent>,
    mut api: ResMut<GzAPI>,
) {
    while let Some(response) = api.try_recv_response() {
        match response {
            GzService::GazeboResourcePathsAdd { .. } => {}
            GzService::GazeboResourcePathsGet { .. } => {}
            GzService::GazeboResourcePathsResolve { .. } => {}
            GzService::GazeboWorlds { .. } => {}
            GzService::GuiCameraViewControl { .. } => {}
            GzService::GuiCameraViewControlReferenceVisual { .. } => {}
            GzService::GuiCameraViewControlSensitivity { .. } => {}
            GzService::GuiCopy { .. } => {}
            GzService::GuiFollow { .. } => {}
            GzService::GuiFollowOffset { .. } => {}
            GzService::GuiMoveTo { .. } => {}
            GzService::GuiMoveToPose { .. } => {}
            GzService::GuiPaste { .. } => {}
            GzService::GuiScreenshot { .. } => {}
            GzService::GuiViewCollisions { .. } => {}
            GzService::GuiViewCom { .. } => {}
            GzService::GuiViewFrames { .. } => {}
            GzService::GuiViewInertia { .. } => {}
            GzService::GuiViewJoints { .. } => {}
            GzService::GuiViewTransparent { .. } => {}
            GzService::GuiViewWireframes { .. } => {}
            GzService::Marker { .. } => {}
            GzService::MarkerList { .. } => {}
            GzService::MarkerArray { .. } => {}
            GzService::ServerControl { .. } => {}
            GzService::WorldControl { response, .. } => {
                if let Some(success) = response {
                    println!("world control: {:?}", success);
                }
            }
            GzService::WorldControlState { .. } => {}
            GzService::WorldCreate { .. } => {}
            GzService::WorldCreateMultiple { .. } => {}
            GzService::WorldDeclareParameter { .. } => {}
            GzService::WorldDisableCollision { .. } => {}
            GzService::WorldEnableCollision { .. } => {}
            GzService::WorldEntitySystemAdd { .. } => {}
            GzService::WorldGenerateWorldSdf { .. } => {}
            GzService::WorldGetParameter { .. } => {}
            GzService::WorldGuiInfo { .. } => {}
            GzService::WorldLevelSetPerformer { .. } => {}
            GzService::WorldLightConfig { .. } => {}
            GzService::WorldListParameters { .. } => {}
            GzService::WorldPlaybackControl { .. } => {}
            GzService::WorldRemove { .. } => {}
            GzService::WorldSceneGraph { .. } => {}
            GzService::WorldSceneInfo { world_name, request:_, response } => {
                if world_name == api.world_name() {
                    if let Some(scene) = response {
                        scene_event.send(SceneEvent::New(scene));
                    }
                }
            }
            GzService::WorldSetParameter { .. } => {}
            GzService::WorldSetPhysics { .. } => {}
            GzService::WorldSetPose { .. } => {}
            GzService::WorldSetPoseVector { .. } => {}
            GzService::WorldSetSphericalCoordinates { .. } => {}
            GzService::WorldState { .. } => {}
            GzService::WorldStateAsync { .. } => {}
            GzService::WorldSystemInfo { .. } => {}
            GzService::WorldVisualConfig { .. } => {}
            GzService::WorldWheelSlip { .. } => {}
            GzService::Unknown => {}
        }
    }

    while let Some(subscription) = api.try_recv_subscription() {
        match subscription {
            GzTopic::Clock(_) => {}
            GzTopic::GazeboResourcePaths(_) => {}
            GzTopic::GuiCameraPose(_) => {}
            GzTopic::Stats(_) => {}
            GzTopic::WorldClock(_, _) => {}
            GzTopic::WorldDynamicPoseInfo(world_name, sub) => {
                if let Some(pose_v) = sub {
                    let mut pose_map = HashMap::<u32, msgs::Pose>::new();
                    for pose in pose_v.pose {
                        pose_map.insert(pose.id, pose);
                    }
                    pose_event.send(PoseEvent { pose_map });
                }
            }
            GzTopic::WorldPoseInfo(_, _) => {}
            GzTopic::WorldSceneDeletion(_, _) => {}
            GzTopic::WorldSceneInfo(_, _) => {}
            GzTopic::WorldState(_, _) => {}
            GzTopic::WorldStats(_, _) => {}
            GzTopic::Unknown => {}
        }
    }
}

fn update_scene(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut clear_color: ResMut<ClearColor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<SceneEvent>,
    mut query: Query<(Entity, &mut GzScene)>,
) {

    for event in events.read() {
        match event {
            SceneEvent::New(scene) => {
                for (parent, _) in &mut query {
                    commands.entity(parent).despawn_recursive();
                }
                spawn_scene(
                    &mut commands,
                    &mut ambient_light,
                    &mut clear_color,
                    &mut meshes,
                    &mut materials,
                    scene.clone(),
                );
            }
            SceneEvent::Update(scene) => {
                // update_scene(&mut commands, &mut meshes, &mut materials, scene.clone());
            }
        }
    }
}

fn update_pose(mut query: Query<(&mut Transform, &GzId)>, mut events: EventReader<PoseEvent>) {
    for event in events.read() {
        for (mut transform, gz_id) in &mut query {
            if let Some(pose) = event.pose_map.get(&gz_id.id) {
                transform.translation = utils::vec3(pose.position.as_ref().unwrap());
                transform.rotation = utils::quat(pose.orientation.as_ref().unwrap());
            }
        }
    }
}
