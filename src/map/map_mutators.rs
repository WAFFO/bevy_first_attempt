use bevy::prelude::*;
use oorandom::Rand32;

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

pub struct PlasmaSquare {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl PlasmaSquare {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        PlasmaSquare { x1, y1, x2, y2 }
    }

    pub fn run_mutate(
        mut height_map: ResMut<BitImage>,
        first_quad: PlasmaSquare,
        mut rand: ResMut<Rand32>,
        scale: f32,
    ) {
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(first_quad);
        while stack.len() > 0 {
            count += 1;
            let quad = stack.pop().unwrap();
            let (x1, y1, x2, y2) = (quad.x1, quad.y1, quad.x2, quad.y2);
            let xa = x1 + (x2 - x1) / 2;
            let ya = y1 + (y2 - y1) / 2;
            let x1y1 = height_map.getX(x1, y1);
            let x2y1 = height_map.getX(x2, y1);
            let x1y2 = height_map.getX(x1, y2);
            let x2y2 = height_map.getX(x2, y2);
            // let x1y1 = if x1y1 == 0. {
            //     rand.rand_float() * 10.
            // } else {
            //     x1y1
            // };
            // let x2y1 = if x2y1 == 0. {
            //     rand.rand_float() * 10.
            // } else {
            //     x2y1
            // };
            // let x1y2 = if x1y2 == 0. {
            //     rand.rand_float() * 10.
            // } else {
            //     x1y2
            // };
            // let x2y2 = if x2y2 == 0. {
            //     rand.rand_float() * 10.
            // } else {
            //     x2y2
            // };
            let avg1 = (x1y1 + x2y1) / 2.; // top
            let avg2 = (x2y1 + x2y2) / 2.; // right
            let avg3 = (x1y2 + x2y2) / 2.; // bottom
            let avg4 = (x1y1 + x1y2) / 2.; // left
            let range = (x2 - x1) as f32 / 2.;
            let dist = rand.rand_float() * range - rand.rand_float() * range;
            let avg5 = (x1y1 + x1y2 + x2y1 + x2y2) / 4. + dist; // middle
            height_map.point_set(xa, y1, avg1);
            height_map.point_set(x2, ya, avg2);
            height_map.point_set(xa, y2, avg3);
            height_map.point_set(x1, ya, avg4);
            height_map.point_set(xa, ya, avg5);
            if xa > x1 && ya > y1 {
                stack.push(PlasmaSquare {
                    x1,
                    y1,
                    x2: xa,
                    y2: ya,
                });
                stack.push(PlasmaSquare {
                    x1: xa,
                    y1,
                    x2,
                    y2: ya,
                });
                stack.push(PlasmaSquare {
                    x1: xa,
                    y1: ya,
                    x2,
                    y2,
                });
                stack.push(PlasmaSquare {
                    x1,
                    y1: ya,
                    x2: xa,
                    y2,
                });
            }
        }
        println!("{} rectangles", count);
    }
}
