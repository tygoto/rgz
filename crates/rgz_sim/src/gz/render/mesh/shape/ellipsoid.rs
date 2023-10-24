use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use std::f32::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Ellipsoid {
    pub x_radius: f32,
    pub y_radius: f32,
    pub z_radius: f32,
    pub subdivisions: u32,
}

impl Default for Ellipsoid {
    fn default() -> Self {
        Ellipsoid {
            x_radius: 1.0,
            y_radius: 1.0,
            z_radius: 1.0,
            subdivisions: 20,
        }
    }
}

impl From<Ellipsoid> for Mesh {
    fn from(ellipsoid: Ellipsoid) -> Self {
        let subdivisions = ellipsoid.subdivisions;

        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for i in 0..=subdivisions {
            let lat = i as f32 / subdivisions as f32 * PI;
            for j in 0..=subdivisions {
                let lon = j as f32 / subdivisions as f32 * 2.0 * PI;

                let x = ellipsoid.x_radius * lat.sin() * lon.cos();
                let y = ellipsoid.y_radius * lat.sin() * lon.sin();
                let z = ellipsoid.z_radius * lat.cos();

                positions.push([x, y, z]);

                if i < subdivisions && j < subdivisions {
                    let index_base = (i * (subdivisions + 1) + j) as u32;
                    indices.push(index_base);
                    indices.push(index_base + 1);
                    indices.push(index_base + subdivisions + 1);
                    indices.push(index_base + 1);
                    indices.push(index_base + subdivisions + 2);
                    indices.push(index_base + subdivisions + 1);
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
