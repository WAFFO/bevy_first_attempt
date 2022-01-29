use bevy::prelude::*;

use crate::{
    generation::{ImageData, ProgressBar},
    map::{average_by_neighbor, BitImage, PerlinNoise},
    terrain::{terrain_build, TerrainMesh, TerrainSettings},
    AppState, RandStruct,
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
                    .with_system(generation_main.before("last"))
                    .with_system(update_progress_bar.label("last"))
                    .with_system(update_image.label("last")),
            )
            .add_system_set(SystemSet::on_enter(AppState::GenDone).with_system(update_progress_bar))
            .add_system_set(SystemSet::on_exit(AppState::GenDone).with_system(reset_tracker));
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker {
            current_stage: 0,
            current_step_progress: 0.,
            max_stage: 4,
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
    rand: ResMut<RandStruct>,
) {
    match tracker.current_stage {
        0 => run_test(tracker),
        1 => run_perlin_noise(heightmap, rand, terrain_settings, tracker),
        2 => run_averaging(heightmap, terrain_settings, tracker),
        3 => terrain_build(
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
    tracker.add_progress(100.);
}

fn run_perlin_noise(
    heightmap: ResMut<BitImage>,
    mut rand: ResMut<RandStruct>,
    terrain_settings: Res<TerrainSettings>,
    mut tracker: ResMut<Tracker>,
) {
    let s = terrain_settings.unit_count;
    let rect = Rect {
        top: 0,
        left: 0,
        bottom: s,
        right: s,
    };
    let mut perlin = PerlinNoise::new(&mut rand);
    perlin.run_mutate(heightmap, rect);
    tracker.add_progress(100.);
}

fn run_averaging(
    mut heightmap: ResMut<BitImage>,
    terrain_settings: Res<TerrainSettings>,
    mut tracker: ResMut<Tracker>,
) {
    let s = terrain_settings.unit_count;
    let rect = Rect {
        top: 0,
        left: 0,
        bottom: s,
        right: s,
    };
    let total = 5;
    let step = 1. / total as f32;
    average_by_neighbor(heightmap.as_mut(), rect);
    tracker.add_progress(step);
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

fn update_image(
    image_handle: Res<ImageData>,
    mut images: ResMut<Assets<Image>>,
    height_map: Res<BitImage>,
    terrain_settings: Res<TerrainSettings>,
) {
    let image = images.get_mut(&image_handle.image_handle);
    if let Some(img) = image {
        img.data.clone_from(
            &height_map
                .convert_to_rgba(terrain_settings.height_scale, terrain_settings.water_height),
        );
    }
}

fn reset_tracker(mut tracker: ResMut<Tracker>) {
    tracker.current_stage = 0;
    tracker.current_step_progress = 0.;
}
