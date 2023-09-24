mod component;
mod spawn;
mod render;
pub(crate) mod utils;
mod api;
mod service;
mod event;

use bevy::{prelude::*};


use crate::gz::event::{PoseEvent, SceneEvent};
use crate::gz::spawn::spawn_scene;
pub(crate) use crate::gz::api::GzAPI;
use crate::gz::component::{GzId, GzScene};


pub struct GzPlugin;
impl Default for GzPlugin {
    fn default() -> Self {
        GzPlugin {

        }
    }
}
impl Plugin for GzPlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        app.init_resource::<GzAPI>();
        app.add_event::<SceneEvent>();
        app.add_event::<PoseEvent>();
        app.add_systems(PreStartup, pre_startup_system);
        app.add_systems(Update, (recv_response, recv_subscription));
        app.add_systems(Update, (update_scene, update_pose));
    }
}
fn pre_startup_system(mut api: ResMut<GzAPI>) {
    api.init();
}

fn recv_response(
    scene_event: EventWriter<SceneEvent>,
    mut api: ResMut<GzAPI>
){
    api.recv_response(scene_event);
}

fn recv_subscription(
    pose_event: EventWriter<PoseEvent>,
    scene_event: EventWriter<SceneEvent>,
    mut api: ResMut<GzAPI>
){
    api.recv_subscription(pose_event, scene_event);
}

fn update_scene(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut clear_color: ResMut<ClearColor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<SceneEvent>,
    mut query: Query<(Entity, &mut GzScene)>
){
    for event in events.iter() {
        match event {
            SceneEvent::Renew(scene) => {
                for (parent, _) in &mut query {
                    commands.entity(parent).despawn_recursive();
                }
                spawn_scene(&mut commands,
                            &mut ambient_light,
                            &mut clear_color,
                            &mut meshes,
                            &mut materials,
                            scene.clone());
            }
            SceneEvent::Update(scene) => {
                // update_scene(&mut commands, &mut meshes, &mut materials, scene.clone());
            }
        }
    }
}


fn update_pose(
    mut query: Query<(&mut Transform, &GzId)>,
    mut events: EventReader<PoseEvent>,
){
    for event in events.iter() {
        for (mut transform, gz_id) in &mut query {
            if let Some(pose) = event.pose_map.get(&gz_id.id) {
                transform.translation = utils::vec3(pose.position.as_ref().unwrap());
                transform.rotation = utils::quat(pose.orientation.as_ref().unwrap());
            }
        }
    }
}