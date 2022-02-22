use crate::{
    mouseclick::MouseClick,
    resources::TextureHandles,
    types::{Position, UnitType},
};
use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_units)
            .add_system(spawn_on_click_system.after("mouseclick"))
            .add_system(update_position_system);
    }
}

pub fn update_position_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.x, pos.y, 0.);
    }
}

fn spawn_unit(
    commands: &mut Commands,
    texture_handles: &TextureHandles,
    t: UnitType,
    pos: Position,
    col: Color,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_handles.0.get(&t).unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(pos.x, pos.y, 0.),
                scale: Vec3::splat(2.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                color: col,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(pos);
}

fn setup_units(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    let enemy_positions = vec![
        (Position { x: -200., y: 160. }, Color::WHITE),
        (Position { x: 0., y: 120. }, Color::YELLOW),
        (Position { x: 220., y: 20. }, Color::CYAN),
        (Position { x: -70., y: -180. }, Color::ORANGE),
        (Position { x: -145., y: 280. }, Color::GREEN),
    ];

    spawn_unit(
        &mut commands,
        &texture_handles,
        UnitType::Player,
        Position { x: 0., y: 0. },
        Color::WHITE,
    );
    enemy_positions.into_iter().for_each(|(pos, col)| {
        spawn_unit(&mut commands, &texture_handles, UnitType::Kobold, pos, col)
    })
}

fn spawn_on_click_system(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    mut ev_mouseclick: EventReader<MouseClick>,
) {
    for click in ev_mouseclick.iter() {
        spawn_unit(
            &mut commands,
            &texture_handles,
            UnitType::Kobold,
            Position {
                x: click.0.x,
                y: click.0.y,
            },
            Color::WHITE,
        )
    }
}
