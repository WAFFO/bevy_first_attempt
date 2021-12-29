use bevy::prelude::*;

mod debug_camera;

use debug_camera::DebugCameraPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldPlugin)
        .add_plugin(DebugCameraPlugin)
        .run();
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Msaa { samples: 1 })
            .add_startup_system(setup.system())
            .add_system(rotate.system());
    }
}

struct Rotates;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.05, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Rotates);
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Rotates>>) {
    for mut t in query.iter_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Y, 0.1))
    }
}
