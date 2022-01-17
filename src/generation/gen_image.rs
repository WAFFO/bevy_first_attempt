use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::generation::MenuData;

pub struct ImageData {
    image_entity: Entity,
    _image_handle: Handle<Image>,
}

pub fn setup_image(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    menu_data: Res<MenuData>,
) {
    // Create a texture with varying shades of red.
    let texture = Image::new_fill(
        Extent3d {
            width: 32,
            height: 32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &(0..(32 * 32))
            .flat_map(|i| {
                vec![
                    0,
                    (if i < 256 { i } else { 255 }) as u8,
                    (if 32 * 32 - i > 256 {
                        0
                    } else {
                        256 + (i - 32 * 32)
                    }) as u8,
                    255,
                ]
            })
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8UnormSrgb,
    );

    // let texture = Image::new_fill(
    //     Extent3d {
    //         width: 32,
    //         height: 32,
    //         depth_or_array_layers: 1,
    //     },
    //     TextureDimension::D2,
    //     &(0..256).flat_map(|i| vec![i as u8]).collect::<Vec<u8>>(),
    //     TextureFormat::R8Unorm,
    // );

    let image_handle = textures.add(texture);

    let image_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    aspect_ratio: Some(1.0),
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
