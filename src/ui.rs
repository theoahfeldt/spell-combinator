use bevy::prelude::*;

use crate::resources::DefaultFont;

/// This example illustrates how to create a button that changes color and text based on its
/// interaction state.

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct ButtonClick;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut ev_buttonclick: EventWriter<ButtonClick>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                ev_buttonclick.send(ButtonClick);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
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
            // Effect Menu
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
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Text",
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
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
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Button",
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
