use bevy::prelude::*;
use bevy::utils::HashMap;
use rgz_msgs as msgs;

#[derive(Event)]
pub(crate) struct PoseEvent {
    pub pose_map: HashMap<u32, msgs::Pose>,
}

#[derive(Event)]
pub(crate) enum SceneEvent {
    New(msgs::Scene),
    Update(msgs::Scene),
}
