use bevy::prelude::*;

use crate::gen_image::{cleanup_image, setup_image};
use crate::AppState;

pub struct GenMenuPlugin;

impl Plugin for GenMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(AppState::PreGenMenu).with_system(setup_menu.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::GenMenu).with_system(setup_image.system()),
            )
            .add_system_set(SystemSet::on_update(AppState::GenMenu).with_system(menu.system()))
            .add_system_set(
                SystemSet::on_exit(AppState::GenMenu)
                    .with_system(cleanup_menu.system())
                    .with_system(cleanup_image.system()),
            );
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub struct MenuData {
    root_node_entity: Entity,
    pub image_node_entity: Entity,
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<State<AppState>>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    let root_node_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: color_materials.add(Color::rgb(0., 0., 0.).into()),
            ..Default::default()
        })
        .id();

    let options_and_image_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Parent(root_node_entity))
        .id();

    let _options_node_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(35.), Val::Percent(100.)),
                ..Default::default()
            },
            material: color_materials.add(Color::rgb(0.24, 0.24, 0.24).into()),
            ..Default::default()
        })
        .insert(Parent(options_and_image_entity))
        .id();

    let image_node_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            material: color_materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            ..Default::default()
        })
        .insert(Parent(options_and_image_entity))
        .id();

    let _button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(65.0)),
                // center button
                margin: Rect {
                    top: Val::Auto,
                    ..Default::default()
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Generate",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(Parent(root_node_entity))
        .id();

    commands.insert_resource(MenuData {
        root_node_entity,
        image_node_entity,
    });

    state.set(AppState::GenMenu).unwrap();
}

fn menu(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands
        .entity(menu_data.root_node_entity)
        .despawn_recursive();
}
