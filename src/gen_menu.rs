use bevy::prelude::*;

use crate::gen_image::{cleanup_image, setup_image};
use crate::AppState;

pub struct GenMenuPlugin;

impl Plugin for GenMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::PreGenMenu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_enter(AppState::GenConfig)
                    .with_system(setup_image)
                    .with_system(update_button_text),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GenConfig).with_system(interact_generate_button),
            )
            .add_system_set(SystemSet::on_enter(AppState::GenRun).with_system(update_button_text))
            .add_system_set(SystemSet::on_enter(AppState::GenDone).with_system(update_button_text))
            .add_system_set(
                SystemSet::on_update(AppState::GenDone).with_system(interact_progress_bar_button),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::GenDone)
                    .with_system(cleanup_menu)
                    .with_system(cleanup_image),
            );
    }
}

#[derive(Component)]
struct GenerateButton;

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct GenerateButtonText;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

const PROG_BAR_NORMAL_BUTTON: Color = Color::rgb(0., 0.60, 0.);
const PROG_BAR_HOVERED_BUTTON: Color = Color::rgb(0., 0.85, 0.);

pub struct MenuData {
    root_node_entity: Entity,
    pub image_node_entity: Entity,
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
            color: Color::rgb(0., 0., 0.).into(),
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
            color: Color::rgb(0.24, 0.24, 0.24).into(),
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
            color: Color::rgb(0.4, 0.4, 0.4).into(),
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
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(GenerateButton)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(0.), Val::Percent(100.)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Percent(0.),
                            right: Val::Auto,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: PROG_BAR_NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(ProgressBar);
            parent
                .spawn_bundle(TextBundle {
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
                })
                .insert(GenerateButtonText);
        })
        .insert(Parent(root_node_entity))
        .id();

    commands.insert_resource(MenuData {
        root_node_entity,
        image_node_entity,
    });

    state.set(AppState::GenConfig).unwrap();
}

fn interact_generate_button(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<GenerateButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::GenRun).unwrap();
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn interact_progress_bar_button(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ProgressBar>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = PROG_BAR_HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = PROG_BAR_NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands
        .entity(menu_data.root_node_entity)
        .despawn_recursive();
}

fn update_button_text(
    state: Res<State<AppState>>,
    mut query: Query<&mut Text, With<GenerateButtonText>>,
) {
    let str = match state.current() {
        AppState::GenConfig => "Generate",
        AppState::GenRun => "Running...",
        AppState::GenDone => "Done - Click to play!",
        _ => "Shouldn't be here....",
    };

    for mut text in query.iter_mut() {
        text.sections[0].value = str.to_string();
    }
}
