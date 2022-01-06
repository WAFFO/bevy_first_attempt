use bevy::prelude::*;
use bevy::render::wireframe::{Wireframe, WireframePlugin};
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};

mod debug_camera;
mod game;
mod gen_image;
mod gen_menu;
mod gen_run;
mod terrain;

use debug_camera::DebugCameraPlugin;
use gen_menu::GenMenuPlugin;
use gen_run::GenRunPlugin;
use terrain::TerrainPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    PreGenMenu,
    GenConfig,
    GenRun,
    GenDone,
    InGame,
}

fn main() {
    App::build()
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                // The Wireframe requires NonFillPolygonMode feature
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .add_state(AppState::PreGenMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(DebugCameraPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(GenMenuPlugin)
        .add_plugin(GenRunPlugin)
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
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.05, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Wireframe)
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
