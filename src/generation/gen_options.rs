use bevy::prelude::*;

use crate::{
    generation::{MenuData, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    randstruct::RandStruct,
    AppState,
};

#[derive(Component)]
pub struct ResetSeedButton;

#[derive(Component)]
pub struct SeedText;

pub fn setup_options(
    mut commands: Commands,
    menu_data: Res<MenuData>,
    asset_server: Res<AssetServer>,
    rand: Res<RandStruct>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::rgb(0.24, 0.24, 0.24).into(),
            ..Default::default()
        })
        .insert(Parent(menu_data.options_node_entity))
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            top: Val::Px(0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("Seed: {}", rand.map_seed()),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Top,
                            horizontal: HorizontalAlign::Left,
                        },
                    ),
                    ..Default::default()
                })
                .insert(SeedText);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Px(35.)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(ResetSeedButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Auto,
                                top: Val::Auto,
                                bottom: Val::Auto,
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "New Seed",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

pub fn interact_reset_seed_button(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ResetSeedButton>),
    >,
    mut text_query: Query<&mut Text, With<SeedText>>,
    mut rand: ResMut<RandStruct>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                rand.randomize_map();
                if *state.current() == AppState::GenDone {
                    state.set(AppState::PreGenMenu).unwrap();
                } else {
                    for mut text in text_query.iter_mut() {
                        text.sections[0].value = format!("Seed: {}", rand.map_seed());
                    }
                }
                *color = PRESSED_BUTTON.into();
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
