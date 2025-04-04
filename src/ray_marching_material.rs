use bevy::{prelude::*, render::{render_asset::RenderAssets, render_resource::{encase, AsBindGroup, OwnedBindingResource, ShaderRef, ShaderType}, renderer::RenderQueue, Extract, Render, RenderApp, RenderSet}, sprite::{Material2d, Material2dPlugin, PreparedMaterial2d}};

use crate::AspectRatio;

pub struct RayMarchingPlugin;

impl Plugin for RayMarchingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CameraMateralData>::default());

        //Add our custom extract and prepare systems to the app
        app.sub_app_mut(RenderApp)
        .add_systems(ExtractSchedule, extract_raymarching_material)
        .add_systems(Render, prepare_raymarching_material.in_set(RenderSet::PrepareResources));
    }
}

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
}

impl CameraMateralData {
    pub fn new() -> CameraMateralData {
        CameraMateralData { 
            camera_position: Vec3::new(0.0, 0.0, 0.0), 
            camera_forward: Vec3::new(0.0, 0.0, -1.0), 
            camera_horizontal: Vec3::new(1.0, 0.0, 0.0), 
            camera_vertical: Vec3::new(0.0, 1.0, 0.0), 
            aspect_ratio: 1.0, 
        }
    }
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

//Uniform data struct to move data from the "Game World" to the "Render World" with the ShaderType derived
#[derive(ShaderType, Clone)]
struct CameraUniformData {
    camera_position: Vec3,
    camera_forward: Vec3,
    camera_horizontal: Vec3,
    camera_vertical: Vec3,
    apsect_ratio: f32,
}

#[derive(ShaderType, Clone)]
struct Spheres {
    #[size(runtime)]
    points: Vec<Vec3>
}

//Move information from the "Game World" to the "Render World"
fn extract_raymarching_material(
    mut commands: Commands,
    ray_marching_query: Extract<Query<(Entity, &MeshMaterial2d<CameraMateralData>)>>,
    aspect_ratio_resource: Extract<Res<AspectRatio>>,
    camera_query: Extract<Query<&Transform, With<Camera2d>>>
) {
    for (entity, material_handle) in ray_marching_query.iter() {
        let mut entity = commands.get_or_spawn(entity);
        entity.insert(material_handle.clone());
        for transform in camera_query.iter() {
            entity.insert(*transform);
        }
    }

    commands.insert_resource(AspectRatio {
        aspect_ratio: aspect_ratio_resource.aspect_ratio,
    });
}

//Update the buffers with the data taken from the "Game World" and sent to the "Render World" so they can be used by the GPU
fn prepare_raymarching_material(
    materials: Res<RenderAssets<PreparedMaterial2d<CameraMateralData>>>,
    material_query: Query<(&Transform, &MeshMaterial2d<CameraMateralData>)>,
    render_queue: Res<RenderQueue>,
    aspect_ratio_resource: Res<AspectRatio>,
) {


    for (transform, material_handle) in &material_query {
        if let Some(material) = materials.get(material_handle) {
            for (i, binding) in material.bindings.iter() {
                if let OwnedBindingResource::Buffer(current_buffer) = binding {
                    let mut buffer = encase::UniformBuffer::new(Vec::new());
                    buffer.write(&CameraUniformData {
                        camera_position: transform.translation,
                        camera_forward: transform.forward().into(),
                        camera_horizontal: transform.right().into(),
                        camera_vertical: transform.up().into(),
                        apsect_ratio: aspect_ratio_resource.aspect_ratio,
                    }).unwrap();
                    //Write to an offset in the buffer so the position data is not over-written
                    render_queue.write_buffer(current_buffer, 0, buffer.as_ref());
                }
            }
        }
    }
}