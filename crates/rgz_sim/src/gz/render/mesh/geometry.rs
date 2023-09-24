use bevy::prelude::*;
use rgz_msgs as msgs;

use super::shape as gz_shape;
use crate::gz::utils;

pub(crate) fn geometry(geometry: Option<&msgs::Geometry>) -> Option<Mesh> {
    let geometry = match geometry {
        Some(geometry) => geometry,
        None => return None,
    };

    let geometry_type = match msgs::geometry::Type::from_i32(geometry.r#type){
        Some(geometry_type) => geometry_type,
        None => return None,
    };

    match geometry_type {
        msgs::geometry::Type::Box => {
            if let Some(msgs::BoxGeom { size: Some(size), header: _, }) = geometry.r#box.as_ref() {
                let size = utils::size3(&size);
                return Some(Mesh::from(shape::Box::new(
                    size.x,
                    size.y,
                    size.z
                )));
            }
        },
        msgs::geometry::Type::Cylinder => {
            if let Some(cylinder_geom) = geometry.cylinder.as_ref() {
                return Some(Mesh::from(shape::Cylinder{
                    radius: cylinder_geom.radius as f32,
                    height: cylinder_geom.length as f32,
                    ..default()
                }));
            }
        },
        msgs::geometry::Type::Sphere => {
            if let Some(sphere_geom) = geometry.sphere.as_ref() {
                return Some(Mesh::from(shape::UVSphere {
                    radius: sphere_geom.radius as f32,
                    ..default()
                }));
            }
        },
        msgs::geometry::Type::Plane => {
            if let Some(plane_geom) = geometry.plane.as_ref() {
                let size = plane_geom.size.as_ref().unwrap();
                let s = size.x.max(size.y);
                return Some(Mesh::from(shape::Plane::from_size(s as f32)));
            }
        },
        msgs::geometry::Type::Image => {
            if let Some(image_geom) = geometry.image.as_ref() {

            }
        },
        msgs::geometry::Type::Heightmap => {
            if let Some(heightmap_geom) = geometry.heightmap.as_ref() {

            }
        },
        msgs::geometry::Type::Mesh => {
            if let Some(mesh_geom) = geometry.mesh.as_ref() {

            }
        },
        msgs::geometry::Type::TriangleFan => {

        },
        msgs::geometry::Type::LineStrip => {

        },
        msgs::geometry::Type::Polyline => {
            for v in &geometry.polyline {

            }
        },
        msgs::geometry::Type::Cone => {
            if let Some(cone_geom) = geometry.cone.as_ref() {

            }
        },
        msgs::geometry::Type::Empty => {},
        msgs::geometry::Type::Arrow => {

        },
        msgs::geometry::Type::Axis => {

        },
        msgs::geometry::Type::Capsule => {
            if let Some(capsule_geom) = geometry.capsule.as_ref() {

                return Some(Mesh::from(shape::Capsule{
                    radius: capsule_geom.radius as f32,
                    depth: capsule_geom.length as f32,
                    ..default()
                }));
            }
        },
        msgs::geometry::Type::Ellipsoid => {
            if let Some(msgs::EllipsoidGeom {radii: Some(radii), header: _} ) = geometry.ellipsoid.as_ref() {
                let radii = utils::vec3(&radii);
                return Some(Mesh::from(gz_shape::Ellipsoid {
                    x_radius: radii.x,
                    y_radius: radii.y,
                    z_radius: radii.z,
                    subdivisions: 20,
                }));
            }
        },
    }
    None
}