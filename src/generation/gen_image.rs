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
    // let size = 32;
    let texture = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &(0..(size * size))
            .flat_map(|i| {
                let x = i % size as u32;
                let y = i / size as u32;
                let v = if x == y || size - x == y { 255u8 } else { 0u8 };
                vec![v, v, v, 255]
            })
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8UnormSrgb,
    );

    let image_handle = textures.add(texture);

    let image_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_wrap: FlexWrap::Wrap,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.)),
                    margin: Rect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    aspect_ratio: Some(1.0),
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
