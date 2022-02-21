use crate::types::Position;
use bevy::prelude::*;

#[derive(Component)]
pub struct Move {
    unit: Entity,
    target: Position,
}

impl Move {
    pub fn new(unit: Entity, target: Position) -> Self {
        Self { unit, target }
    }

    pub fn update(&self, position: &mut Position) {
        position.x += 0.1;
    }
}

pub fn move_system(moves: Query<&Move>, mut pos: Query<&mut Position>) {
    for m in moves.iter() {
        m.update(&mut pos.get_mut(m.unit).unwrap())
    }
}

pub fn add_move_system(mut commands: Commands, query: Query<Entity, With<Position>>) {
    for e in query.iter() {
        commands.spawn().insert(Move {
            unit: e,
            target: Position { x: 0.0, y: 0.0 },
        });
    }
}

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_system).add_system(add_move_system);
    }
}
