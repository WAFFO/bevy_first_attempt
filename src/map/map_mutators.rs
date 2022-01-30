use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

use crate::{map::BitImage, randstruct::RandStruct};

#[derive(Component)]
pub struct ReverseRain {
    x: usize,
    y: usize,
    strength: f32,
    next_coords: Option<(usize, usize)>,
}

#[allow(dead_code)]
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

pub struct PerlinNoise {
    height_noise: Perlin,
}

impl PerlinNoise {
    pub fn new(rand: &mut RandStruct) -> Self {
        PerlinNoise {
            height_noise: Perlin::new().set_seed(rand.get_map_u32()),
        }
    }

    pub fn run_mutate(&mut self, mut height_map: ResMut<BitImage>, area: Rect<usize>) {
        let width = (area.right - area.left) as f64;
        let height = (area.bottom - area.top) as f64;
        let base_frequency = 5.;
        for x in area.left..(area.right + 1) {
            for y in area.top..(area.bottom + 1) {
                let nx = x as f64 / width - 0.5;
                let ny = y as f64 / height - 0.5;
                let d = (2. * nx.abs().max(ny.abs())).powf(2.);
                let nx = (nx) * base_frequency;
                let ny = (ny) * base_frequency;
                let e = self.get_height(nx, ny)
                    + 0.53 * self.get_height(2. * nx, 2. * ny)
                    + 0.20 * self.get_height(4. * nx, 4. * ny)
                    + 0.12 * self.get_height(8. * nx, 8. * ny)
                    + 0.05 * self.get_height(32. * nx, 32. * ny);
                let e = e / (1. + 0.53 + 0.20 + 0.12 + 0.05);
                let e = (0.9 + e - d) / 2.;
                let e = e.powf(4.5);
                height_map.point_set(x, y, e as f32);
            }
        }
    }

    fn get_height(&mut self, x: f64, y: f64) -> f64 {
        self.height_noise.get([x, y]) / 2. + 0.5
    }
}

pub fn average_by_neighbor(height_map: &mut BitImage, area: Rect<usize>) {
    for x in area.left..(area.right + 1) {
        for y in area.top..(area.bottom + 1) {
            let (total, sum) =
                height_map.reduce_neighbors(x, y, (0, 0.), |(count, sum), val: f32| {
                    (count + 1, sum + val)
                });
            let average = sum / total as f32;
            height_map.point_set(x, y, average);
        }
    }
}

pub fn zero_edges(height_map: &mut BitImage, area: Rect<usize>) {
    let mut fun = |x, y| {
        height_map.point_set(x, y, 0.);
    };
    for x in area.left..(area.right + 1) {
        if x == area.left || x == area.right {
            for y in area.top..(area.bottom + 1) {
                fun(x, y);
            }
        } else {
            for y in [area.top, area.bottom] {
                fun(x, y);
            }
        };
    }
}
