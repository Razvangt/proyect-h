use bevy::app::{App, PluginGroup};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::CursorIcon::Default;
use bevy_editor_pls::EditorPlugin;

#[derive(Component)]
struct Cube;
#[derive(Component)]
struct Camera;
#[derive(Component)]
struct cam_tag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_startup_system(camera_system)
        .add_startup_system(begin_game)
        .add_system(move_system)
        .run();
}

pub fn camera_system(mut commands : Commands){
    let trans =  Transform::from_xyz(5., 5., 5.);
    commands.spawn(Camera3dBundle{
        transform : trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(Camera);
    commands.spawn(SpotLightBundle{
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn begin_game( mut commands: Commands,
               mut mesh_assets: ResMut<Assets<Mesh>>,
               mut materials_assets: ResMut<Assets<StandardMaterial>>,){
    commands.spawn(PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials_assets.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.5, 0.5),
                ..default()
            }),
            transform : Transform::from_xyz(0.0,1.0,0.0),
            ..default()
        }).insert(Cube).insert(cam_tag);
    commands.spawn(PbrBundle{
        mesh : mesh_assets.add(Mesh::from(shape::Plane { size : 20.0})),
        material : materials_assets.add(StandardMaterial{
            base_color: Color::rgb(0.5,1.0,0.5),
            ..default()
        }),
        ..default()
    });
}


//  Move camera if cam_tag moves
fn camera_follow_tag_System(mut commands: Commands,mut cameras: Query<&mut Transform, With<Camera>>){
        // I need to know if tag moved so its an envent
}

fn move_system(input: Res<Input<KeyCode>>, time: Res<Time>, mut cubes: Query<&mut Transform, With<Cube>>) {
    for mut transform in cubes.iter_mut() {
        let mut change = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);
        for key in input.get_pressed() {
            match key {
                KeyCode::W => change += forward,
                KeyCode::S => change -= forward,
                KeyCode::A => change -= right,
                KeyCode::D => change += right,
           //     KeyCode::Space => change += Vec3::Y,
           //     KeyCode::LShift => change -= Vec3::Y,
                _ => (),
            }
        }
        change = change.normalize_or_zero();
        transform.translation += change  * time.delta_seconds() * 1.5;
    }
}