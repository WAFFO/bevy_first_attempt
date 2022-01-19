use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};
use oorandom::Rand32;

mod debug_camera;
mod game;
mod generation;
mod map;
mod terrain;

use debug_camera::DebugCameraPlugin;
use game::GamePlugin;
use generation::GenMenuPlugin;
use generation::GenRunPlugin;
use map::WorldDataPlugin;
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
    App::new()
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Rand32::new(420))
        .add_state(AppState::PreGenMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(DebugCameraPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(GenMenuPlugin)
        .add_plugin(GenRunPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(WorldDataPlugin)
        .run();
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 1 })
            .add_startup_system(setup)
            .add_system(rotate)
            .add_system(toggle_wireframe);
    }
}

#[derive(Component)]
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
        .insert(Rotates);
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Rotates>>) {
    for mut t in query.iter_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Y, 0.1))
    }
}

fn toggle_wireframe(
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    let window = windows.get_primary().unwrap();
    for key in keys.get_just_pressed() {
        if window.cursor_locked() {
            match key {
                KeyCode::Backslash => wireframe_config.global = !wireframe_config.global,
                _ => (),
            }
        }
    }
}
