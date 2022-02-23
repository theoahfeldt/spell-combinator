use crate::{
    resources::TextureHandles,
    types::{Health, Position, UnitType},
};
use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_units)
            .add_system(update_position_system);
    }
}

pub fn update_position_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}

#[derive(Bundle)]
struct UnitBundle {
    health: Health,
    position: Position,
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
    commands.spawn_bundle(UnitBundle {
        health: Health(10),
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

// fn spawn_on_click_system(
//     mut commands: Commands,
//     texture_handles: Res<TextureHandles>,
//     mut ev_mouseclick: EventReader<MouseClick>,
// ) {
//     for click in ev_mouseclick.iter() {
//         spawn_unit(
//             &mut commands,
//             &texture_handles,
//             UnitType::Kobold,
//             Position(click.0),
//             Color::WHITE,
//         )
//     }
// }
