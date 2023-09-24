
fn main() {
    // App::new()
    //     // plugins are registered as part of the "app building" process
    //     .add_plugins((
    //         DefaultPlugins,
    //         GzPlugin,
    //         PanOrbitCameraPlugin
    //     ))
    //     .add_plugins(DebugGridPlugin::with_floor_grid())
    //     .insert_resource(AmbientLight {
    //         color: Color::WHITE,
    //         brightness: 1.0,
    //     })
    //     .insert_resource(ClearColor(Color::BLACK))
    //     .add_systems(Startup, startup_system)
    //     // .add_systems(Update, (
    //     //     update_system,
    //     //     ))
    //     .run();
}
//
// pub(crate) fn startup_system(
//     mut commands: Commands,
//     mut api: ResMut<gz::GzAPI>,
// ) {
//     api.set_world_name("sensor_world");
//     api.init_scene();
//
//     let camera_pose = rgz_msgs::Pose {
//         position: Some(rgz_msgs::Vector3d {
//             x: -6.0,
//             y: 0.0,
//             z: 6.0,
//             ..default()
//         }),
//         orientation: Some(rgz_msgs::Quaternion {
//             x: 0.0,
//             y: 0.24740396440029144,
//             z: 0.0,
//             w: 0.96891242265701294,
//             ..default()
//         }),
//         ..default()
//     };
//
//     // Camera
//     commands.spawn((
//         Camera3dBundle {
//             transform: utils::pose(Some(&camera_pose), None),
//             ..default()
//         },
//         PanOrbitCamera::default(),
//     ));
// }





