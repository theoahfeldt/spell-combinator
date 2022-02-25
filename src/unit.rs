use crate::{
    effect::Effects,
    resources::TextureHandles,
    types::{Health, Position, UnitType},
};
use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_units)
            .add_system(update_effect_system)
            .add_system(update_position_system);
    }
}

pub fn update_position_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}

pub fn update_effect_system(mut query: Query<(&mut Health, &mut Position, &mut Effects)>) {
    for (mut health, mut pos, mut effects) in query.iter_mut() {
        if let Some(effect) = effects.0.get(0) {
            if effect.update(&mut health, &mut pos) {
                effects.0.pop_front();
            }
        }
    }
}

pub struct Unit {
    pub entity: Entity,
    pub health: Health,
    pub position: Position,
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
) {
    let builder = &mut commands.spawn_bundle(UnitBundle {
        health: Health(10),
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
    });
    if t == UnitType::Player {
        builder.insert(Player);
    }
}

fn setup_units(mut commands: Commands, texture_handles: Res<TextureHandles>) {
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
    );
    enemy_positions.into_iter().for_each(|(pos, col)| {
        spawn_unit(&mut commands, &texture_handles, UnitType::Kobold, pos, col)
    })
}
