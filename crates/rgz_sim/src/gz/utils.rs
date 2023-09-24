use bevy::prelude::*;
use rgz_msgs as msgs;

pub(crate) fn vec3(vector3d: &msgs::Vector3d) -> Vec3 {
    // Vec3::new(vector3d.x as f32, vector3d.y as f32, vector3d.z as f32)
    Vec3::new(-vector3d.y as f32, vector3d.z as f32, -vector3d.x as f32)
}

pub(crate) fn size3(size: &msgs::Vector3d) -> Vec3 {
    Vec3::new(size.y.abs() as f32, size.z.abs() as f32, size.x.abs() as f32)
}

pub(crate) fn scale3(scale: &msgs::Vector3d) -> Vec3 {
    Vec3::new(scale.y.abs() as f32, scale.z.abs() as f32, scale.x.abs() as f32)
}

pub(crate) fn quat(quaternion: &msgs::Quaternion) -> Quat {
    // Quat::from_xyzw(quaternion.x as f32, quaternion.y as f32, quaternion.z as f32, quaternion.w as f32)
    Quat::from_xyzw(-quaternion.y as f32, quaternion.z as f32, -quaternion.x as f32, quaternion.w as f32)
}

pub(crate) fn pose(pose: Option<&msgs::Pose>, scale: Option<&msgs::Vector3d>) -> Transform {
    match pose {
        Some(pose) => {
            let translation = match pose.position.as_ref() {
                Some(position) => vec3(position),
                None => Vec3::default()
            };

            let rotation = match pose.orientation.as_ref() {
                Some(orientation) => quat(orientation),
                None => Quat::IDENTITY,
            };

            let scaling = match scale {
                Some(scale) => scale3(scale),
                None => Vec3::ONE,
            };

            Transform {
                translation,
                rotation,
                scale: scaling,
                ..default()
            }
        },
        None => Transform::default()
    }
}

pub(crate) fn color(color: Option<&msgs::Color>) -> Color {
    match color {
        Some(color) => Color::rgba(color.r, color.g, color.b, color.a),
        None => Color::default()
    }
}