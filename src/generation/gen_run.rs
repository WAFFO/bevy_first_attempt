use bevy::prelude::*;
use oorandom::Rand32;

use crate::{
    generation::ProgressBar,
    map::{BitImage, PlasmaSquare},
    terrain::{terrain_build, TerrainMesh, TerrainSettings},
    AppState,
};

pub struct GenRunPlugin;

pub struct Tracker {
    pub current_stage: u32,
    pub current_step_progress: f32,
    pub max_stage: u32,
}

impl Plugin for GenRunPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tracker>()
            .add_system_set(
                SystemSet::on_update(AppState::GenRun)
                    .with_system(generation_main)
                    .with_system(update_progress_bar),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::GenDone).with_system(update_progress_bar),
            );
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker {
            current_stage: 0,
            current_step_progress: 0.,
            max_stage: 3,
        }
    }
}

/////////////// main function for generation

fn generation_main(
    tracker: ResMut<Tracker>,
    heightmap: ResMut<BitImage>,
    terrain_settings: Res<TerrainSettings>,
    terrain_data: Res<TerrainMesh>,
    meshes: ResMut<Assets<Mesh>>,
    state: ResMut<State<AppState>>,
    rand: ResMut<Rand32>,
) {
    match tracker.current_stage {
        0 => run_test(tracker),
        1 => run_plasma_setup(heightmap, rand, terrain_settings, tracker),
        2 => terrain_build(
            terrain_settings,
            terrain_data,
            heightmap.as_ref(),
            meshes,
            tracker,
        ),
        _ => end_generation(state),
    }
}

/////////////// start: run functions for generation

fn run_test(mut tracker: ResMut<Tracker>) {
    tracker.add_progress(0.1);
}

fn run_plasma_setup(
    mut heightmap: ResMut<BitImage>,
    mut rand: ResMut<Rand32>,
    terrain_settings: Res<TerrainSettings>,
    mut tracker: ResMut<Tracker>,
) {
    let s = terrain_settings.unit_count;
    let quad = PlasmaSquare::new(0, 0, s, s);
    let scale = 10.;
    heightmap.point_set(0, 0, rand.rand_float() * scale);
    heightmap.point_set(0, s, rand.rand_float() * scale);
    heightmap.point_set(s, 0, rand.rand_float() * scale);
    heightmap.point_set(s, s, rand.rand_float() * scale);
    PlasmaSquare::run_mutate(heightmap, quad, rand, scale);
    tracker.add_progress(100.);
}

fn end_generation(mut state: ResMut<State<AppState>>) {
    state.set(AppState::GenDone).unwrap();
}

/////////////// end: run functions for generation

impl Tracker {
    pub fn add_progress(&mut self, progress: f32) {
        self.current_step_progress += progress;
        if self.current_step_progress >= 1.0 {
            self.current_step_progress = 0.0;
            self.current_stage += 1;
        }
    }
}

fn update_progress_bar(tracker: Res<Tracker>, mut query: Query<&mut Style, With<ProgressBar>>) {
    let c = tracker.current_stage as f32;
    let m = tracker.max_stage as f32;
    let s = tracker.current_step_progress;
    let p = (c / m + s / m) * 100.;
    for mut style in query.iter_mut() {
        style.size.width = Val::Percent(p);
    }
}
