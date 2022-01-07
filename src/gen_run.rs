use bevy::prelude::*;

use crate::gen_menu::ProgressBar;
use crate::terrain::terrain_build;
use crate::AppState;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GenState {
    Off,
    Test,
    TestMesh,
    Done,
}

pub struct GenRunPlugin;

pub struct Tracker {
    pub current_stage: u32,
    pub current_step_progress: f32,
    pub max_stage: u32,
}

impl Plugin for GenRunPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Tracker>()
            .add_state(GenState::Off)
            .add_system_set(
                SystemSet::on_enter(AppState::GenRun).with_system(start_generation.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GenRun).with_system(update_progress_bar.system()),
            )
            .add_system_set(SystemSet::on_update(GenState::Test).with_system(run_test.system()))
            .add_system_set(
                SystemSet::on_enter(GenState::TestMesh).with_system(terrain_build.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GenState::Done).with_system(end_generation.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::GenDone).with_system(update_progress_bar.system()),
            );
    }
}

fn start_generation(mut tracker: ResMut<Tracker>, state: ResMut<State<GenState>>) {
    tracker.add_progress(100., state);
}

/////////////// start: run functions for generation

fn run_test(mut tracker: ResMut<Tracker>, state: ResMut<State<GenState>>) {
    tracker.add_progress(0.1, state);
}

/////////////// end: run functions for generation

fn end_generation(mut tracker: ResMut<Tracker>, mut state: ResMut<State<AppState>>) {
    tracker.current_step_progress = 0.;
    tracker.current_stage = tracker.max_stage;
    state.set(AppState::GenDone).unwrap();
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

fn stage_to_state(stage: u32) -> GenState {
    match stage {
        0 => GenState::Off,
        1 => GenState::Test,
        2 => GenState::TestMesh,
        _ => GenState::Done,
    }
}

impl Tracker {
    pub fn add_progress(&mut self, progress: f32, mut state: ResMut<State<GenState>>) {
        self.current_step_progress += progress;
        if self.current_step_progress >= 1.0 {
            self.current_step_progress = 0.0;
            self.current_stage += 1;
            state.set(stage_to_state(self.current_stage)).unwrap();
        }
    }
}

fn update_progress_bar(tracker: Res<Tracker>, mut query: Query<&mut Style, With<ProgressBar>>) {
    let c = tracker.current_stage as f32 - 1.0;
    let m = tracker.max_stage as f32 - 1.0;
    let s = tracker.current_step_progress;
    let p = (c / m + s / m) * 100.;
    for mut style in query.iter_mut() {
        style.size.width = Val::Percent(p);
    }
}
