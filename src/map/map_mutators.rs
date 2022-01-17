use bevy::prelude::*;

use crate::map::BitImage;

#[derive(Component)]
pub struct ReverseRain {
    x: usize,
    y: usize,
    strength: f32,
    next_coords: Option<(usize, usize)>,
}

impl ReverseRain {
    pub fn new(x: usize, y: usize, strength: f32) -> Self {
        ReverseRain {
            x,
            y,
            strength,
            next_coords: Some((x, y)),
        }
    }

    pub fn merge(a: ReverseRain, b: ReverseRain) -> ReverseRain {
        ReverseRain {
            x: a.x,
            y: a.y,
            strength: a.strength + b.strength,
            next_coords: None,
        }
    }

    pub fn run_mutate(
        mut commands: Commands,
        mut query: Query<(&mut ReverseRain, Entity)>,
        mut height_map: ResMut<BitImage>,
    ) {
        for (mut drop, entity) in query.iter_mut() {
            if let Some((x, y)) = drop.next_coords {
                drop.x = x;
                drop.y = y;
                height_map.point_raise(drop.x, drop.y, drop.strength);
                height_map.neighbor_raise(drop.x, drop.y, drop.strength / 2.);
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    pub fn run_check(mut query: Query<&mut ReverseRain>, height_map: Res<BitImage>) {
        for mut drop in query.iter_mut() {
            drop.next_coords = height_map.compare_to_neighbors(drop.x, drop.y, f32::gt);
        }
    }
}
