use bevy::app::{App, PluginGroup};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::render::camera::ScalingMode;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Cube;
#[derive(Component)]
struct Camera;
#[derive(Component)]
struct CamTag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(camera_system)
        .add_startup_system(begin_game)
        .add_system(move_system)
        .add_system(camera_follow_tag_system)
        .run();
}

pub fn camera_system(mut commands: Commands) {
    let trans = Transform::from_xyz(10., 10., 10.0);
    commands
        .spawn(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
        .into(),
            transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Camera);
    commands.spawn(SpotLightBundle {
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn begin_game(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials_assets: ResMut<Assets<StandardMaterial>>,
) {

    //Player
    commands
        .spawn(PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials_assets.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(Cube)
        .insert(CamTag).insert(KinematicCharacterController::default());

    //Map
    commands.spawn(PbrBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials_assets.add(StandardMaterial {
            base_color: Color::rgb(0.5, 1.0, 0.5),
            ..default()
        }),
        ..default()
    }).insert(Collider::cuboid(20.0, 0.1, 20.0));
    commands.spawn(PbrBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 5.0 })),
        material: materials_assets.add(StandardMaterial {
            base_color: Color::rgb(2., 2.0, 5.),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, -5.0),
        ..default()
    }).insert(Collider::cuboid(5.0, 5., 5.0));
    commands.spawn(PbrBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials_assets.add(StandardMaterial {
            base_color: Color::rgb(2., 2.0, 5.),
            ..default()
        }),
        transform: Transform::from_xyz(6.0, 1.0, -5.0),
        ..default()
    });

}




//  Move camera if cam_tag moves
fn camera_follow_tag_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    // I need to know if tag moved so its an envent

    for mut transform in camera.iter_mut() {
        let mut change = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., 0.0);
        let right = Vec3::new(local_z.z, 0., 0.0);
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
        transform.translation += change * time.delta_seconds() * 1.5;
    }
}

fn move_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut controllers : Query<&mut KinematicCharacterController>,
    mut cubes: Query<&mut Transform, With<Cube>>,
) {
    for mut controller in controllers.iter_mut() {
        let mut change = Vec3::ZERO;
        for key in input.get_pressed() {
            let x_movement = Vec3::new(1., 0., 0.);
            let z_movement = Vec3::new(0., 0., 1.);
            match key {
                KeyCode::W => change += x_movement,
                KeyCode::S => change -= x_movement,
                KeyCode::A => change -= z_movement,
                KeyCode::D => change += z_movement,
                //     KeyCode::Space => change += Vec3::Y,
                //     KeyCode::LShift => change -= Vec3::Y,
                _ => (),
            }
        }
        change = change.normalize_or_zero();
        controller.translation = Some(change * time.delta_seconds() * 1.5);
    }

    for mut transform in cubes.iter_mut() {
        let mut change = Vec3::ZERO;
        let mut change = Vec3::ZERO;
        for key in input.get_pressed() {
            let x_movement = Vec3::new(1., 0., 0.);
            let z_movement = Vec3::new(0., 0., 1.);
            match key {
                KeyCode::W => change += x_movement,
                KeyCode::S => change -= x_movement,
                KeyCode::A => change -= z_movement,
                KeyCode::D => change += z_movement,
                //     KeyCode::Space => change += Vec3::Y,
                //     KeyCode::LShift => change -= Vec3::Y,
                _ => (),
            }
        }
        change = change.normalize_or_zero();
    //    transform.translation += change * time.delta_seconds() * 1.5;
    }
}

