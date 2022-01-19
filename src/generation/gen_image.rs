use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{generation::MenuData, terrain::TerrainSettings};

pub struct ImageData {
    image_entity: Entity,
    _image_handle: Handle<Image>,
}

pub fn setup_image(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    menu_data: Res<MenuData>,
    // height_map: Res<BitImage>,
    terrain_settings: Res<TerrainSettings>,
) {
    let size = terrain_settings.unit_count as u32;
    // Create a texture with varying shades of red.
    let texture = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &(0..(size * size))
            .flat_map(|i| vec![0, (i % 255) as u8, 0, 255])
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8UnormSrgb,
    );

    let image_handle = textures.add(texture);

    let image_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.)),
                    aspect_ratio: Some(1.0),
                    flex_shrink: 0.001,
                    flex_basis: Val::Percent(100.),
                    overflow: Overflow::Hidden,
                    ..Default::default()
                },
                image: image_handle.clone().into(),
                ..Default::default()
            });
        })
        .insert(Parent(menu_data.image_node_entity))
        .id();
    commands.insert_resource(ImageData {
        image_entity,
        _image_handle: image_handle,
    });
}

pub fn cleanup_image(mut commands: Commands, image_data: Res<ImageData>) {
    commands.entity(image_data.image_entity).despawn_recursive();
}
