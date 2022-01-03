use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};

pub struct ImageData {
    camera_entity: Entity,
    image_entity: Entity,
    _image_handle: Handle<Texture>,
}

pub fn setup_image(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a texture with varying shades of red.
    let texture = Texture::new_fill(
        Extent3d {
            width: 16,
            height: 16,
            depth: 1,
        },
        TextureDimension::D2,
        &(0..(256))
            .flat_map(|i| vec![255, i as u8, 0, 255])
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8UnormSrgb,
    );

    let image_handle = textures.add(texture);

    let camera_entity = commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .id();

    let image_entity = commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(image_handle.clone().into()),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .id();
    commands.insert_resource(ImageData {
        camera_entity,
        image_entity,
        _image_handle: image_handle,
    });
}

pub fn cleanup_image(mut commands: Commands, image_data: Res<ImageData>) {
    commands
        .entity(image_data.camera_entity)
        .despawn_recursive();
    commands.entity(image_data.image_entity).despawn_recursive();
}
