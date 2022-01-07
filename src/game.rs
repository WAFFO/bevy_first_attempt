use bevy::prelude::*;

use crate::terrain::terrain_startup;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(terrain_startup.system()),
        );
    }
}
