use bevy::prelude::*;

use crate::{effect::MovePrep, mouseclick::MouseClick, resources::DefaultFont, types::Position};

/// This example illustrates how to create a button that changes color and text based on its
/// interaction state.

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct ButtonClick;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_button_system)
            .add_system(select_target_system)
            .add_system(update_move_menu_system);
    }
}

fn select_target_system(
    mut ev_mouseclick: EventReader<MouseClick>,
    query: Query<(Entity, &Position)>,
    mut button_query: Query<&mut MovePrep, With<Button>>,
) {
    let prep: &mut MovePrep = &mut button_query.iter_mut().next().unwrap();
    for click in ev_mouseclick.iter() {
        let mut unit = None;
        for (e, pos) in query.iter() {
            if pos.0.distance(click.0) < 5. {
                unit = Some(e);
            }
        }
        if let Some(_) = unit {
            prep.unit = unit;
        } else {
            prep.target = Some(Position(click.0));
        }
    }
}

fn update_move_menu_system(
    mut query: Query<(&Name, &mut Text)>,
    button_query: Query<&MovePrep, With<Button>>,
) {
    let prep = button_query.iter().next().unwrap();
    for (name, ref mut text) in query.iter_mut() {
        if *name == Name::new("MoveText") {
            text.sections[0].value = format!("Unit: {:?}\n Target: {:?}", prep.unit, prep.target);
        }
    }
}

fn move_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MovePrep),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, prep) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                if let Some(m) = prep.compile() {
                    commands.spawn().insert(m);
                }
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

fn setup(mut commands: Commands, font: Res<DefaultFont>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        // Root node, covers entire window
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Move Menu
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                        align_self: AlignSelf::FlexEnd,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Unit: None\n Target: None",
                                TextStyle {
                                    font: font.0.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new("MoveText"));
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                // center button
                                margin: Rect::all(Val::Auto),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..Default::default()
                        })
                        .insert(MovePrep::new())
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Execute",
                                    TextStyle {
                                        font: font.0.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        });
}
