use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};

use crate::{generation::Tracker, map::BitImage};

pub struct TerrainPlugin;

pub struct TerrainSettings {
    pub unit_count: usize,
    pub unit_size: f32,
    pub height_scale: f32,
    pub water_height: f32,
}

pub struct TerrainMesh {
    pub mesh_handle: Handle<Mesh>,
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainSettings>()
            .init_resource::<TerrainMesh>();
    }
}

impl FromWorld for TerrainMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        TerrainMesh {
            mesh_handle: meshes.add(Mesh::new(PrimitiveTopology::TriangleList)),
        }
    }
}

impl Default for TerrainSettings {
    fn default() -> Self {
        TerrainSettings {
            unit_count: 1024,
            unit_size: 1.,
            height_scale: 300.,
            water_height: 5.,
        }
    }
}

pub fn terrain_startup(
    mut commands: Commands,
    terrain_data: Res<TerrainMesh>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    terrain_settings: Res<TerrainSettings>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: terrain_data.mesh_handle.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            metallic: 0.,
            reflectance: 0.1,
            perceptual_roughness: 0.9,
            ..Default::default()
        }),
        ..Default::default()
    });

    let limit = 100000.;

    // water plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: limit })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgba(0.3, 0.4, 1., 0.95),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0., terrain_settings.water_height, 0.),
        ..Default::default()
    });

    // ground plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: limit })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            metallic: 0.,
            reflectance: 0.1,
            perceptual_roughness: 0.9,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}

pub fn terrain_build(
    terrain_settings: Res<TerrainSettings>,
    terrain_data: Res<TerrainMesh>,
    heightmap: &BitImage,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tracker: ResMut<Tracker>,
) {
    let size = terrain_settings.unit_count;
    let unit_size = terrain_settings.unit_size;
    let mesh = meshes.get_mut(terrain_data.mesh_handle.clone()).unwrap();

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let vertex_number = (size + 1).pow(2);

    vertices.resize(vertex_number, [0.0f32, 0.0f32, 0.0f32]);
    normals.resize(vertex_number, [0.0f32, 1.0f32, 0.0f32]);
    // TODO: update UV coords to correct 00, 01, 10, 11 coords
    let uvs = vec![[0.0, 0.0]; vertices.len()];

    // vertex
    let mut vertex_index = 0;
    for cy in 0..(size + 1) {
        for cx in 0..(size + 1) {
            // do height here (debug wave)
            // let h = ((cx + cy) as f32 / 4.).sin();
            let h = heightmap.get(cx, cy).unwrap() * unit_size * terrain_settings.height_scale;
            vertices[vertex_index] = [cx as f32 * unit_size, h, cy as f32 * unit_size];
            vertex_index += 1;
        }
    }

    // index
    let grid_width = size as u32 + 1;
    for cy in 0..(size as u32) {
        for cx in 0..(size as u32) {
            let ltr = 1; //(cx + cy + 1) % 2;
            let rtl = ltr ^ 1;
            indices.extend(
                [
                    cy * grid_width + cx,
                    (cy + 1) * grid_width + cx + 1 * ltr,
                    cy * grid_width + cx + 1,
                ]
                .iter(),
            );
            indices.extend(
                [
                    cy * grid_width + cx + 1 * rtl,
                    (cy + 1) * grid_width + cx,
                    (cy + 1) * grid_width + cx + 1,
                ]
                .iter(),
            );
        }
    }

    // normal
    for i in (2..indices.len() - 3).step_by(3) {
        let p = (
            vertices[indices[i - 2] as usize],
            vertices[indices[i - 1] as usize],
            vertices[indices[i] as usize],
        );
        let u = (p.1[0] - p.0[0], p.1[1] - p.0[1], p.1[2] - p.0[2]);
        let v = (p.2[0] - p.0[0], p.2[1] - p.0[1], p.2[2] - p.0[2]);
        let n = (
            (u.1 * v.2) - (u.2 * v.1),
            (u.2 * v.0) - (u.0 * v.2),
            (u.0 * v.1) - (u.1 * v.0),
        );
        let len = (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt();
        normals[indices[i - 2] as usize] = [n.0 / len, n.1 / len, n.2 / len];
        normals[indices[i - 1] as usize] = [n.0 / len, n.1 / len, n.2 / len];
        normals[indices[i] as usize] = [n.0 / len, n.1 / len, n.2 / len];
    }

    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(vertices),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    tracker.add_progress(100.);
}
