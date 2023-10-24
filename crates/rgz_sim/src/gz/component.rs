use bevy::prelude::*;
use rgz_msgs as msgs;

#[derive(Component)]
pub(crate) struct GzId {
    pub(crate) name: String,
    pub(crate) id: u32,
}

#[derive(Component)]
pub(crate) struct GzScene;
impl GzScene {
    pub(crate) fn new(scene: msgs::Scene) -> Self {
        Self
    }
}

#[derive(Component)]
pub(crate) struct GzModel;
impl GzModel {
    pub(crate) fn new(model: msgs::Model) -> Self {
        Self
    }
}

#[derive(Component)]
pub(crate) struct GzLink;
impl GzLink {
    pub(crate) fn new(link: msgs::Link) -> Self {
        Self
    }
}

#[derive(Component)]
pub(crate) struct GzJoint;
impl GzJoint {
    pub(crate) fn new(joint: msgs::Joint) -> Self {
        Self
    }
}

#[derive(Component)]
pub(crate) struct GzVisual;
impl GzVisual {
    pub(crate) fn new(visual: msgs::Visual) -> Self {
        Self
    }
}

#[derive(Component)]
pub(crate) struct GzLight;
impl GzLight {
    pub(crate) fn new(light: msgs::Light) -> Self {
        Self
    }
}
