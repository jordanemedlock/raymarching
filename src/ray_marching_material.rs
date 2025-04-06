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

    #[storage(1, read_only)]
    pub points: Handle<ShaderStorageBuffer>,
}

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
