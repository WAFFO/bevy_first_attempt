use bevy::app::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

struct DebugCamera;
pub struct DebugCameraPlugin;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

/// Mouse sensitivity and movement speed
pub struct CameraSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InputState>()
            .init_resource::<CameraSettings>()
            .add_startup_system(setup_grab_cursor.system())
            .add_startup_system(setup_camera.system())
            .add_system(cursor_grab.system())
            .add_system(camera_look.system());
    }
}

/// Grabs the cursor when game first starts
fn setup_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

/// Spawns the `Camera3dBundle` to be controlled
fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(DebugCamera);
}

/// Grabs/releases mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

// system to check if the user wants to release the window
fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}

/// Handles looking around if cursor is locked
fn camera_look(
    settings: Res<CameraSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<(&DebugCamera, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    for (_camera, mut transform) in query.iter_mut() {
        for event in state.reader_motion.iter(&motion) {
            if window.cursor_locked() {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());

                state.pitch -= (settings.sensitivity * event.delta.y * window_scale).to_radians();
                state.yaw -= (settings.sensitivity * event.delta.x * window_scale).to_radians();
            }
            state.pitch = state.pitch.clamp(-1.54, 1.54);
            // Order is important, turn and then look up/down vs looking up/down and turning (causes roll)
            transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw)
                * Quat::from_axis_angle(Vec3::X, state.pitch);
        }
    }
}
