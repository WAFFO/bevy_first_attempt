use bevy::app::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
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
            sensitivity: 0.00024,
            speed: 8.,
        }
    }
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<CameraSettings>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(setup_grab_cursor)
                    .with_system(setup_camera),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(cursor_grab)
                    .with_system(camera_look)
                    .with_system(camera_move),
            );
    }
}

/// Grabs the cursor when game first starts
fn setup_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

/// Spawns the `Camera3dBundle` to be controlled
fn setup_camera(mut commands: Commands, mut state: ResMut<InputState>) {
    let transform = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    let q = &transform.rotation;
    state.yaw =
        (2.0 * (q.y * q.z + q.w * q.x)).atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z);
    state.pitch = (-2.0 * (q.x * q.z - q.w * q.y)).asin();
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .insert(DebugCamera)
        .with_children(|parent| {
            parent.spawn_bundle(PointLightBundle {
                point_light: PointLight {
                    intensity: 3200.0,
                    range: 400.,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
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

/// Handles keyboard input and movement
fn camera_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<CameraSettings>,
    mut query: Query<(&DebugCamera, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);
        let mut sprint = 1.0f32;

        for key in keys.get_pressed() {
            if window.cursor_locked() {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= right,
                    KeyCode::D => velocity += right,
                    KeyCode::R => velocity += Vec3::Y,
                    KeyCode::F => velocity -= Vec3::Y,
                    KeyCode::LShift => sprint = 4.,
                    _ => (),
                }
            }
        }
        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * time.delta_seconds() * settings.speed * sprint
    }
}
