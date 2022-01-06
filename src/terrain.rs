use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
        wireframe::Wireframe,
    },
};

pub struct TerrainPlugin;
pub struct TerrainSettings {
    pub unit_count: usize,
    pub unit_size: f32,
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<TerrainSettings>();
    }
}

impl Default for TerrainSettings {
    fn default() -> Self {
        TerrainSettings {
            unit_count: 1024,
            unit_size: 1.,
        }
    }
}

pub fn terrain_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_settings: Res<TerrainSettings>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(terrain_build(
                terrain_settings.unit_count,
                terrain_settings.unit_size,
            )),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Wireframe);
}

pub fn terrain_build(size: usize, unit_size: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let vertex_number = (size + 1).pow(2);

    vertices.resize(vertex_number, [0.0f32, 0.0f32, 0.0f32]);
    normals.resize(vertex_number, [0.0f32, 1.0f32, 0.0f32]);
    // TODO: update UV coords to correct 00, 01, 10, 11 coords
    let uvs = vec![[0.0, 0.0, 0.0]; vertices.len()];

    // vertex
    let mut vertex_index = 0;
    for cy in 0..(size + 1) {
        for cx in 0..(size + 1) {
            // do height here
            let h = ((cx + cy) as f32 / 4.).sin();
            vertices[vertex_index] = [cx as f32 * unit_size, h, cy as f32 * unit_size];
            vertex_index += 1;
        }
    }

    // index
    let grid_width = size as u32 + 1;
    for cy in 0..(size as u32) {
        for cx in 0..(size as u32) {
            let ltr = (cx + cy + 1) % 2;
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
        VertexAttributeValues::Float3(vertices),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float3(normals),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float3(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
