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
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
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

pub struct SpellBuilderPlugin;

impl Plugin for SpellBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
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
