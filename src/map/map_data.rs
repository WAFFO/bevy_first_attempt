use bevy::prelude::*;

use crate::{
    map::{HeightMapIter, HeightMapNormIter},
    terrain::TerrainSettings,
};

pub struct WorldDataPlugin;

impl Plugin for WorldDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_bit_world_map);
    }
}

fn init_bit_world_map(mut commands: Commands, terrain_settings: Res<TerrainSettings>) {
    commands.insert_resource(BitImage::new(terrain_settings.unit_count));
}

pub struct BitImage {
    data: Vec<f32>,
    edge_size: usize,
    max_height: f32,
    min_height: f32,
}

#[allow(dead_code)]
impl BitImage {
    fn new(edge_size: usize) -> Self {
        let len = edge_size + 1;
        BitImage {
            data: vec![0.; len * len],
            edge_size: len,
            max_height: 0.,
            min_height: 0.,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<f32, String> {
        self.check_coords(x, y)?;
        Ok(self.data[y * self.edge_size + x] as f32)
    }

    pub fn get_ignore(&self, x: usize, y: usize) -> f32 {
        self.get(x, y).unwrap_or_default()
    }

    pub fn get_normalized(&self, x: usize, y: usize) -> Result<f32, String> {
        self.check_coords(x, y)?;
        Ok((self.data[y * self.edge_size + x] - self.min_height)
            / (self.max_height - self.min_height))
    }

    pub fn get_heightmap_iter(&self) -> HeightMapIter {
        HeightMapIter::new(&self.data)
    }

    pub fn get_heightmap_norm_iter(&self) -> HeightMapNormIter {
        HeightMapNormIter::new(&self.data, self.max_height, self.min_height)
    }

    pub fn point_raise(&mut self, x: usize, y: usize, val: f32) {
        if let Err(_) = self.check_coords(x, y) {
            return;
        }

        let c = self.data[y * self.edge_size + x] + val;

        if c > self.max_height {
            self.max_height = c;
        } else if c < self.min_height {
            self.min_height = c;
        }

        self.data[y * self.edge_size + x] = c;
    }

    pub fn point_set(&mut self, x: usize, y: usize, val: f32) {
        if let Err(e) = self.check_coords(x, y) {
            println!("{}", e);
            return;
        }

        if val > self.max_height {
            self.max_height = val;
        } else if val < self.min_height {
            self.min_height = val;
        }

        self.data[y * self.edge_size + x] = val;
    }

    pub fn neighbor_raise(&mut self, x: usize, y: usize, val: f32) {
        let neighbors = Self::get_neighbors(x, y);
        for coord in neighbors {
            self.point_raise(coord.0, coord.1, val);
        }
    }

    pub fn compare_to_neighbors<F>(&self, x: usize, y: usize, compare: F) -> Option<(usize, usize)>
    where
        F: Fn(&f32, &f32) -> bool,
    {
        let neighbors = Self::get_neighbors(x, y);
        let mut max = self.get(x, y).unwrap();
        let mut max_coord = None;
        for coord in neighbors {
            let current = self.get(coord.0, coord.1).unwrap_or_default();
            if compare(&current, &max) {
                max = current;
                max_coord = Some(coord)
            }
        }
        max_coord
    }

    fn get_neighbors(x: usize, y: usize) -> [(usize, usize); 8] {
        let y0 = if y > 0 { y - 1 } else { y };
        let x0 = if x > 0 { x - 1 } else { x };
        [
            (x0, y0),
            (x, y0),
            (x + 1, y0),
            (x0, y),
            (x + 1, y),
            (x0, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
    }

    fn check_coords(&self, x: usize, y: usize) -> Result<(), String> {
        if x >= self.edge_size {
            return Err(format!(
                "Out of bounds: x = {}, width = {}",
                x, self.edge_size
            ));
        }
        if y >= self.edge_size {
            return Err(format!(
                "Out of bounds: y = {}, height = {}",
                y, self.edge_size
            ));
        }

        Ok(())
    }

    // TODO :(
    pub fn convert_to_rgba(&self) -> Vec<u8> {
        let mut vec = vec![0; self.edge_size * self.edge_size * 4];

        for (i, data) in self.get_heightmap_norm_iter().enumerate() {
            let idx = i * 4;
            let val = (data * 255.) as u8;
            vec[idx] = val;
            vec[idx + 1] = val;
            vec[idx + 2] = val;
            vec[idx + 3] = 255;
        }
        vec
    }
}
