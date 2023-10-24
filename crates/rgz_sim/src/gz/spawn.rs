use crate::gz::component::{GzId, GzLight, GzLink, GzModel, GzScene, GzVisual};
use crate::gz::render;
use crate::gz::utils;
use bevy::prelude::*;
use bevy::ui::AlignItems::Default;
use rgz_msgs as msgs;

pub(super) fn spawn_scene(
    commands: &mut Commands,
    ambient_light: &mut ResMut<AmbientLight>,
    clear_color: &mut ResMut<ClearColor>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    scene: msgs::Scene,
) {
    // ambient
    // ambient_light.color = utils::color(scene.ambient.as_ref());

    // background
    clear_color.0 = utils::color(scene.background.as_ref());

    // grid
    // scene.grid;

    let parent = commands
        .spawn((
            PbrBundle {
                visibility: Visibility::Hidden,
                ..default()
            },
            GzScene::new(scene.clone()),
        ))
        .id();

    for light in scene.light {
        // scene.shadows;
        let child = spawn_light(commands, light);
        if let Some(child) = child {
            commands.entity(parent).add_child(child);
        }
    }

    for model in scene.model {
        let child = spawn_model(commands, meshes, materials, model);
        if let Some(child) = child {
            commands.entity(parent).add_child(child);
        }
    }
}

fn spawn_light(commands: &mut Commands, light: msgs::Light) -> Option<Entity> {
    let light_type = match msgs::light::LightType::from_i32(light.r#type) {
        Some(light_type) => light_type,
        None => return None,
    };
    let transform = utils::pose(light.pose.as_ref(), None);

    let entity = match light_type {
        msgs::light::LightType::Point => commands
            .spawn((
                PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1.0, 1.0, 1.0),
                        intensity: 1500.0,
                        range: 0.0,
                        radius: 0.0,
                        shadows_enabled: true,
                        shadow_depth_bias: 0.0,
                        shadow_normal_bias: 0.0,
                    },
                    transform,
                    ..default()
                },
                GzId {
                    name: light.name.clone(),
                    id: light.id,
                },
                GzLight::new(light.clone()),
            ))
            .id(),
        msgs::light::LightType::Spot => commands
            .spawn((
                SpotLightBundle {
                    spot_light: SpotLight {
                        color: Color::rgb(1.0, 1.0, 1.0),
                        intensity: 1500.0,
                        range: 0.0,
                        radius: 0.0,
                        shadows_enabled: true,
                        shadow_depth_bias: 0.0,
                        shadow_normal_bias: 0.0,
                        outer_angle: 0.0,
                        inner_angle: 0.0,
                    },
                    transform,
                    ..default()
                },
                GzId {
                    name: light.name.clone(),
                    id: light.id,
                },
                GzLight::new(light.clone()),
            ))
            .id(),
        msgs::light::LightType::Directional => commands
            .spawn((
                DirectionalLightBundle {
                    directional_light: DirectionalLight { ..default() },
                    transform,
                    ..default()
                },
                GzId {
                    name: light.name.clone(),
                    id: light.id,
                },
                GzLight::new(light.clone()),
            ))
            .id(),
    };
    Some(entity)
}

fn spawn_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    model: msgs::Model,
) -> Option<Entity> {
    let parent = commands
        .spawn((
            PbrBundle {
                // mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                transform: utils::pose(model.pose.as_ref(), model.scale.as_ref()),
                visibility: Visibility::Hidden,
                ..default()
            },
            GzId {
                name: model.name.clone(),
                id: model.id,
            },
            GzModel::new(model.clone()),
        ))
        .id();

    for m in model.model {
        let child = spawn_model(commands, meshes, materials, m);
        if let Some(child) = child {
            commands.entity(parent).add_child(child);
        }
    }

    for l in model.link {
        let child = spawn_link(commands, meshes, materials, l);
        if let Some(child) = child {
            commands.entity(parent).add_child(child);
        }
    }

    Some(parent)
}

fn spawn_link(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    link: msgs::Link,
) -> Option<Entity> {
    let parent = commands
        .spawn((
            PbrBundle {
                transform: utils::pose(link.pose.as_ref(), None),
                visibility: Visibility::Hidden,
                ..default()
            },
            GzId {
                name: link.name.clone(),
                id: link.id,
            },
            GzLink::new(link.clone()),
        ))
        .id();

    for v in link.visual {
        let child = spawn_visual(commands, meshes, materials, v);
        if let Some(child) = child {
            commands.entity(parent).add_child(child);
        }
    }

    Some(parent)
}

fn spawn_visual(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    visual: msgs::Visual,
) -> Option<Entity> {
    // let visual_type = match msgs::visual::Type::from_i32(visual.r#type){
    //     Some(visual_type) => visual_type,
    //     None => return None,
    // };

    let mesh = match render::mesh::geometry(visual.geometry.as_ref()) {
        Some(mesh) => mesh,
        None => return None,
    };

    let transform = utils::pose(visual.pose.as_ref(), visual.scale.as_ref());
    let material = render::material(visual.material.as_ref());

    let parent = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(material),
                visibility: Visibility::Visible,
                transform,
                ..default()
            },
            GzId {
                name: visual.name.clone(),
                id: visual.id,
            },
            GzVisual::new(visual.clone()),
        ))
        .id();

    Some(parent)
}
