use bevy::prelude::*;
use rgz_msgs as msgs;

pub(crate) fn material(
    material: Option<&msgs::Material>
) -> StandardMaterial {

    if let Some(material) = material {
        // StandardMaterial {
        //     base_color: Color::rgba(0.5, 0.5, 0.5, 0.3),
        //     base_color_texture: None,
        //     emissive: Color::default(),
        //     emissive_texture: None,
        //     perceptual_roughness: 0.0,
        //     metallic: 0.0,
        //     metallic_roughness_texture: None,
        //     reflectance: 0.0,
        //     normal_map_texture: None,
        //     flip_normal_map_y: false,
        //     occlusion_texture: None,
        //     double_sided: false,
        //     cull_mode: None,
        //     unlit: false,
        //     fog_enabled: false,
        //     alpha_mode: AlphaMode::Multiply,
        //     depth_bias: 0.0,
        //     depth_map: None,
        //     parallax_depth_scale: 0.0,
        //     // parallax_mapping_method: ParallaxMappingMethod:,
        //     max_parallax_layer_count: 0.0,
        //     ..default()
        // }
        StandardMaterial {
            // base_color: Color::rgba(0.1, 0.1, 0.1, 0.1),
            base_color: match material.diffuse.as_ref() {
                Some(diffuse) => Color::rgba(diffuse.r, diffuse.g, diffuse.b, diffuse.a),
                None => Color::rgb(0.1, 0.1, 0.1),
            },
            ..default()
        }
    }else{
        StandardMaterial {
            ..default()
        }
    }
}