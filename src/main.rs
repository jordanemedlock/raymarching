use std::default;

use bevy::{asset::RenderAssetUsages, prelude::*, render::{camera, render_resource::{Extent3d, TextureDimension, TextureFormat}, storage::ShaderStorageBuffer}, sprite::{Material2dPlugin, MaterialMesh2dBundle}, window::{PrimaryWindow, WindowResized, WindowResolution}};
use bevy_inspector_egui::quick::*;
use bevy::input::mouse::MouseMotion;


use bevy_flycam::prelude::*;
// use ray_marching_material::RayMarchingPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod screen_space_quad;
use crate::screen_space_quad::ScreenSpaceQuad;

mod ray_marching_material;
use crate::ray_marching_material::CameraMateralData;

//Struct which becomes the Global Resource for the aspect ratio
#[derive(Resource, Reflect, Default)]
pub struct AspectRatio {
    aspect_ratio: f32,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [1280.0, 720.0].into(),
                title: "Ray Marching Test".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<AspectRatio>()

        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<AspectRatio>::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(Material2dPlugin::<CameraMateralData>::default())
        // .add_plugins(NoCameraPlayerPlugin)
        // .add_plugins(RayMarchingPlugin)
        // .insert_resource(MovementSettings {
        //     sensitivity: 0.00005, // default: 0.00012
        //     speed: 12.0, // default: 12.0
        // })
        // .insert_resource(KeyBindings {
        //     move_ascend: KeyCode::Space,
        //     move_descend: KeyCode::ShiftLeft,
        //     ..Default::default()
        // })
        .add_systems(Startup, setup)
        .add_systems(Update, (resize_event, process_camera_rotation, process_camera_translation))
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CameraMateralData>>,
    mut images: ResMut<Assets<Image>>,
) {

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0)
    ));
    // Camera2dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 5.0),
    //     ..default()
    // });
    let data:Vec<u8> = (0..10*10*10).map(|i| {
        if i < 100 && i % 3 == 0 { 255 } else { 0 }
    }).collect::<Vec<u8>>(); 
    let image = Image::new(
        Extent3d { width: 10, height: 10, depth_or_array_layers: 10 }, 
        TextureDimension::D3, 
        data, 
        TextureFormat::R8Unorm,
        RenderAssetUsages::all()
    );

    let grid_handle = images.add(image);
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(ScreenSpaceQuad::default())).into()),
        MeshMaterial2d(materials.add(CameraMateralData {
            camera_position: Vec3::new(0.0, 0.0, 0.0), 
            camera_forward: Vec3::new(0.0, 0.0, -1.0), 
            camera_horizontal: Vec3::new(1.0, 0.0, 0.0), 
            camera_vertical: Vec3::new(0.0, 1.0, 0.0), 
            aspect_ratio: 1.0, 
            grid: grid_handle,
        }))
    ));
    // MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(ScreenSpaceQuad::default())).into(),
    //     material: materials.add(CameraMateralData::new()),
    //     ..default()
    // });
}

fn resize_event( 
    mut resize_reader: EventReader<WindowResized>,
    mesh_material_query: Query<&MeshMaterial2d<CameraMateralData>>,
    mut materials: ResMut<Assets<CameraMateralData>>,
) {
    let mesh_material = mesh_material_query.single();
    if let Some(camera_data) = materials.get_mut(mesh_material) {
        for event in resize_reader.read() {
            camera_data.aspect_ratio = event.width / event.height;
            println!("updating aspect ratio");
        }
    }
}


fn process_camera_translation(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mesh_material_query: Query<&MeshMaterial2d<CameraMateralData>>,
    mut materials: ResMut<Assets<CameraMateralData>>,
    time: Res<Time>, 
) {
    const SPEED: f32 = 1.0;
    for mut transform in camera_query.iter_mut() {
        let forward_vector = transform.forward();
        let horizontal_vector = transform.right();
        let vertical_vector = transform.up();
        if keys.pressed(KeyCode::KeyW) {
            transform.translation += forward_vector * SPEED * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation -= forward_vector * SPEED * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation -= horizontal_vector * SPEED * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation += horizontal_vector * SPEED * time.delta_secs();
        }
        if keys.pressed(KeyCode::Space) {
            transform.translation += vertical_vector * SPEED * time.delta_secs();
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            transform.translation -= vertical_vector * SPEED * time.delta_secs();
        }

        let mesh_material = mesh_material_query.single();
        if let Some(camera_data) = materials.get_mut(mesh_material) {
            camera_data.camera_position = transform.translation;
            camera_data.camera_forward = transform.forward().into();
            camera_data.camera_horizontal = transform.right().into();
            camera_data.camera_vertical = transform.up().into();
        }
    }
}

fn process_camera_rotation(
    mut motion_event: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>
) {
    for event in motion_event.read() {
        const ROTATION_SPEED: f32 = 0.1;
        if mouse_buttons.pressed(MouseButton::Right) {
            for mut transform in camera_query.iter_mut() {
                transform.rotate_local_x(-event.delta.y * ROTATION_SPEED * time.delta_secs());
                transform.rotate_local_y(-event.delta.x * ROTATION_SPEED * time.delta_secs());
            }
        }
    }
}

// fn move_balls(
//     mesh_material_query: Query<&MeshMaterial2d<CameraMateralData>>,
//     mut materials: ResMut<Assets<CameraMateralData>>,
//     mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
//     time: Res<Time>, 
// ) {
//     let mesh_material = mesh_material_query.single();
//     let material = materials.get_mut(&mesh_material.0).unwrap();
//     let buffer = buffers.get_mut(&material.points).unwrap();
//     buffer.set_data((0..5).map(|i| {
//         let t = time.elapsed_secs();
//         [
//             2.0 * ops::sin(t + i as f32) + 0.5,
//             2.0 * ops::sin(t + i as f32 + 2.0) + 0.5,
//             2.0 * ops::sin(t + i as f32 + 4.0) + 0.5,
//             1.0,
//         ]
//     })
//     .collect::<Vec<[f32; 4]>>()
//     .as_slice())
// }