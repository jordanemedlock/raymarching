use bevy::{prelude::*, render::{render_asset::RenderAssets, render_resource::{encase, AsBindGroup, OwnedBindingResource, ShaderRef, ShaderType}, renderer::RenderQueue, storage::ShaderStorageBuffer, Extract, Render, RenderApp, RenderSet}, sprite::{Material2d, Material2dPlugin, PreparedMaterial2d}};

use crate::AspectRatio;

//New material created to setup custom shader
#[derive(AsBindGroup, Debug, Clone, TypePath, Asset)]
pub struct CameraMateralData {
    //Set the uniform at binding 0 to have the following information - connects to Camera struct in ray_marching_material.wgsl
    #[uniform(0)]
    pub camera_position: Vec3,
    #[uniform(0)]
    pub camera_forward: Vec3,
    #[uniform(0)]
    pub camera_horizontal: Vec3,
    #[uniform(0)]
    pub camera_vertical: Vec3,
    #[uniform(0)]
    pub aspect_ratio: f32,

    #[texture(1, dimension="3d")]
    #[sampler(2)]
    pub grid: Handle<Image>,
}

// impl Default for CameraMateralData {
//     fn default() -> Self { CameraMateralData::new() }
// }

// impl CameraMateralData {
//     pub fn new() -> CameraMateralData {
//         CameraMateralData { 
//             camera_position: Vec3::new(0.0, 0.0, 0.0), 
//             camera_forward: Vec3::new(0.0, 0.0, -1.0), 
//             camera_horizontal: Vec3::new(1.0, 0.0, 0.0), 
//             camera_vertical: Vec3::new(0.0, 1.0, 0.0), 
//             aspect_ratio: 1.0, 
//         }
//     }
// }

//Setup the CameraMateralData to use the custom shader file for the vertex and fragment shader
//Note: one of these can be removed to use the default material 2D bevy shaders for the vertex/fragment shader
impl Material2d for CameraMateralData {
    fn vertex_shader() -> ShaderRef {
        "shaders/ray_marching_material.wgsl".into()    
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ray_marching_material.wgsl".into()
    }
}
