use crate::{
    effect::Effects,
    resources::{DefaultFont, TextureHandles},
    types::{Health, Position, UnitType},
};
use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_units)
            .add_system(update_effect)
            .add_system(update_transform)
            .add_system(update_health_text);
    }
}

fn update_transform(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}

// Runs the effects in each units effects queue one after the other
fn update_effect(mut query: Query<(&mut Health, &mut Position, &mut Effects)>) {
    for (mut health, mut pos, mut effects) in query.iter_mut() {
        if let Some(effect) = effects.0.get(0) {
            if effect.update(&mut health, &mut pos) {
                effects.0.pop_front();
            }
        }
    }
}

#[derive(Component)]
struct HealthText;

fn update_health_text(
    mut q_text: Query<&mut Text, With<HealthText>>,
    q_unit: Query<(&Health, &Children), Changed<Health>>,
) {
    for (health, children) in q_unit.iter() {
        for &child in children.iter() {
            if let Ok(mut text) = q_text.get_mut(child) {
                text.sections[0].value = health.0.to_string();
            }
        }
    }
}

fn health_text(health: i32, font: Handle<Font>) -> Text {
    Text::with_section(
        health.to_string(),
        TextStyle {
            font,
            font_size: 10.0,
            color: Color::RED,
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    )
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct UnitBundle {
    health: Health,
    position: Position,
    effects: Effects,
    #[bundle]
    sprite: SpriteSheetBundle,
    timer: Timer,
}

fn spawn_unit(
    commands: &mut Commands,
    texture_handles: &TextureHandles,
    t: UnitType,
    pos: Position,
    col: Color,
    health: i32,
    font: Handle<Font>,
) {
    let entity = commands
        .spawn_bundle(UnitBundle {
            health: Health(health),
            effects: Effects::new(),
            sprite: SpriteSheetBundle {
                texture_atlas: texture_handles.0.get(&t).unwrap().clone(),
                transform: Transform {
                    translation: pos.0.extend(0.),
                    scale: Vec3::splat(2.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    color: col,
                    ..Default::default()
                },
                ..Default::default()
            },
            position: pos,
            timer: Timer::from_seconds(0.1, true),
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(Text2dBundle {
                    text: health_text(health, font),
                    transform: Transform::from_translation(Vec3::new(0., -20., 0.)),
                    ..Default::default()
                })
                .insert(HealthText);
        })
        .id();
    if t == UnitType::Player {
        commands.entity(entity).insert(Player);
    }
}

fn setup_units(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    font: Res<DefaultFont>,
) {
    let enemy_positions = vec![
        (Position(Vec2::new(-200., 160.)), Color::WHITE),
        (Position(Vec2::new(0., 120.)), Color::YELLOW),
        (Position(Vec2::new(220., 20.)), Color::CYAN),
        (Position(Vec2::new(-70., -180.)), Color::ORANGE),
        (Position(Vec2::new(-145., 280.)), Color::GREEN),
    ];

    spawn_unit(
        &mut commands,
        &texture_handles,
        UnitType::Player,
        Position(Vec2::ZERO),
        Color::WHITE,
        30,
        font.0.clone(),
    );
    enemy_positions.into_iter().for_each(|(pos, col)| {
        spawn_unit(
            &mut commands,
            &texture_handles,
            UnitType::Kobold,
            pos,
            col,
            10,
            font.0.clone(),
        )
    })
}
