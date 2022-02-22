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

    pub fn update(&self, position: &mut Position) -> bool {
        let diff = self.target.0 - position.0;
        if diff.length() < 1. {
            position.0 = self.target.0;
            true
        } else {
            position.0 += diff.normalize();
            false
        }
    }
}

#[derive(Component)]
pub struct MovePrep {
    pub unit: Option<Entity>,
    pub target: Option<Position>,
}

impl MovePrep {
    pub fn new() -> Self {
        Self {
            unit: None,
            target: None,
        }
    }

    pub fn compile(&self) -> Option<Move> {
        self.unit
            .and_then(|u| self.target.clone().map(|t| Move::new(u, t)))
    }
}

pub fn move_system(
    mut commands: Commands,
    moves: Query<(Entity, &Move)>,
    mut pos: Query<&mut Position>,
) {
    for (e, m) in moves.iter() {
        if m.update(&mut pos.get_mut(m.unit).unwrap()) {
            commands.entity(e).despawn();
        }
    }
}

// pub fn add_move_system(
//     mut commands: Commands,
//     query: Query<Entity, With<Position>>,
//     mut ev_buttonclick: EventReader<ButtonClick>,
// ) {
//     for _click in ev_buttonclick.iter() {
//         for e in query.iter() {
//             commands.spawn().insert(Move {
//                 unit: e,
//                 target: Position { x: 0.0, y: 0.0 },
//             });
//         }
//     }
// }

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_system);
    }
}
