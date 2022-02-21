use bevy::prelude::*;

pub struct Target {
    pub center: Position,
    pub radius: f32,
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum UnitType {
    Player,
    Kobold,
}
