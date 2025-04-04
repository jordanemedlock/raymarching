use std::default;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::{PrimaryWindow, WindowResized, WindowResolution}};
use bevy_inspector_egui::quick::*;
use bevy::input::mouse::MouseMotion;


use bevy_flycam::prelude::*;
use ray_marching_material::RayMarchingPlugin;


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
        .add_plugins(ResourceInspectorPlugin::<AspectRatio>::default())
        .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(RayMarchingPlugin)
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
) {

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(ScreenSpaceQuad::default())).into(),
        material: materials.add(CameraMateralData::new()),
        ..default()
    });
}

fn resize_event( 
    mut resize_reader: EventReader<WindowResized>,
    mut aspect_ratio_resource: ResMut<AspectRatio>,
) {
    for event in resize_reader.read() {
        aspect_ratio_resource.aspect_ratio = event.width / event.height;
        println!("updating aspect ratio");
    }
}


fn process_camera_translation(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>, 
) {
    const SPEED: f32 = 1.0;
    for mut transform in camera_query.iter_mut() {
        let forward_vector = transform.forward();
        let horizontal_vector = transform.right();
        let vertical_vector = transform.up();
        if keys.pressed(KeyCode::KeyW) {
            transform.translation += forward_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation -= forward_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation -= horizontal_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation += horizontal_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyR) {
            transform.translation += vertical_vector * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyF) {
            transform.translation -= vertical_vector * SPEED * time.delta_seconds();
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
                transform.rotate_local_x(-event.delta.y * ROTATION_SPEED * time.delta_seconds());
                transform.rotate_local_y(-event.delta.x * ROTATION_SPEED * time.delta_seconds());
            }
        }
    }
}