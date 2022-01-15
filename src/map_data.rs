use bevy::prelude::*;

use crate::terrain::TerrainSettings;

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
}

#[allow(dead_code)]
impl BitImage {
    fn new(edge_size: usize) -> Self {
        let len = edge_size + 1;
        BitImage {
            data: vec![0.; len * len],
            edge_size: len,
            max_height: 0.,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<f32, String> {
        self.check_coords(x, y)?;
        Ok(self.data[y * self.edge_size + x] as f32)
    }

    pub fn get_normalized(&self, x: usize, y: usize) -> Result<f32, String> {
        self.check_coords(x, y)?;
        Ok(self.data[y * self.edge_size + x] / self.max_height)
    }

    pub fn point_raise(&mut self, x: usize, y: usize, val: f32) -> Result<(), String> {
        self.check_coords(x, y)?;

        let c = self.data[y * self.edge_size + x] + val;

        if c > self.max_height {
            self.max_height = c;
        }

        self.data[y * self.edge_size + x] = c;

        Ok(())
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
    pub fn convert_rgba(&self, width: usize, height: usize) -> Vec<u8> {
        let vec = vec![0; width * height];

        vec
    }
}
