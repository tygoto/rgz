use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::{AppArgs, gz};

pub fn run(
    world: String,
    running: bool,
    verbose: u8,
){

    info!("run");

    let app_args = AppArgs {
        world,
        running
    };

    let level = match verbose {
        1 => Some(bevy::log::Level::ERROR),
        2 => Some(bevy::log::Level::WARN),
        3 => Some(bevy::log::Level::INFO),
        4 => Some(bevy::log::Level::DEBUG),
        _ => None,
    };

    let mut app = App::new();

    if let Some(level) = level {
        app.add_plugins(
            DefaultPlugins.set(
                LogPlugin {
                    level,
                    ..default()
                }
            ),
        );
    }else{
        app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
    }

    app.add_plugins((
            gz::GzPlugin,
            PanOrbitCameraPlugin
        ))
        // .add_plugins(DebugGridPlugin::with_floor_grid())
        .insert_resource(app_args)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, startup_system)
        // .add_systems(Update, (
        //     update_system,
        //     ))
        .run();
}

pub(crate) fn startup_system(
    mut commands: Commands,
    mut api: ResMut<gz::GzAPI>,
    mut app_args: ResMut<AppArgs>,
) {

    api.set_world(app_args.world.as_str());

    if app_args.running {
        api.play();
    }

    let camera_pose = rgz_msgs::Pose {
        position: Some(rgz_msgs::Vector3d {
            x: -6.0,
            y: 0.0,
            z: 6.0,
            ..default()
        }),
        orientation: Some(rgz_msgs::Quaternion {
            x: 0.0,
            y: 0.24740396440029144,
            z: 0.0,
            w: 0.96891242265701294,
            ..default()
        }),
        ..default()
    };

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: gz::utils::pose(Some(&camera_pose), None),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
