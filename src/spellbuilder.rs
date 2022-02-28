use bevy::prelude::*;

use crate::resources::DefaultFont;

struct SpellCard {
    position: Vec2,
    name: String,
    description: String,
}

impl SpellCard {
    fn spawn(self, mut commands: Commands, font: Res<DefaultFont>, root: Entity) {
        let Self {
            position,
            name,
            description,
        } = self;
        let child = commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    position: Rect {
                        left: Val::Px(position.x),
                        bottom: Val::Px(position.y),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(100.), Val::Px(100.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::rgb(0.6, 0.7, 0.2).into(),
                ..Default::default()
            })
            .insert(SpellCardTag)
            .with_children(|parent| {
                // Title
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        name,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 20.,
                            color: Color::rgb(0., 0., 0.),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
                // Description
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        description,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 16.,
                            color: Color::rgb(0., 0., 0.),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            })
            .id();
        commands.entity(root).push_children(&[child]);
    }
}

#[derive(Component)]
struct Selected(Vec2);

#[derive(Component)]
struct SpellCardTag;

fn select_card_system(
    mut commands: Commands,
    windows: Res<Windows>,
    interaction_query: Query<
        (Entity, &Style, &Interaction),
        (Changed<Interaction>, With<SpellCardTag>),
    >,
) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        for (card, style, interaction) in interaction_query.iter() {
            if let Interaction::Clicked = interaction {
                if let (Val::Px(x), Val::Px(y)) = (style.position.left, style.position.bottom) {
                    commands
                        .entity(card)
                        .insert(Selected(Vec2::new(x - position.x, y - position.y)));
                }
            } else {
                commands.entity(card).remove::<Selected>();
            }
        }
    }
}

fn move_card_system(
    mut cursor_evr: EventReader<CursorMoved>,
    mut card_query: Query<(&mut Style, &Selected), With<SpellCardTag>>,
) {
    for (mut style, Selected(pos)) in card_query.iter_mut() {
        for ev in cursor_evr.iter() {
            style.position.left = Val::Px(ev.position.x + pos.x);
            style.position.bottom = Val::Px(ev.position.y + pos.y)
        }
    }
}

pub struct SpellBuilderPlugin;

impl Plugin for SpellBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(select_card_system)
            .add_system(move_card_system);
    }
}

#[derive(Component)]
pub struct SpellBuilderUI;

fn setup(mut commands: Commands, font: Res<DefaultFont>) {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::None,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.5).into(),
            ..Default::default()
        })
        .insert(SpellBuilderUI)
        .id();
    let card = SpellCard {
        position: Vec2::new(50.0, 50.0),
        name: "Epic Spell".to_string(),
        description: "This spell is epic.".to_string(),
    };
    card.spawn(commands, font, root);
}
