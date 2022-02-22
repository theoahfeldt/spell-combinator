use bevy::prelude::*;

pub struct Target {
    pub center: Position,
    pub radius: f32,
}

#[derive(Component, Clone, Debug)]
pub struct Position(pub Vec2);

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum UnitType {
    Player,
    Kobold,
}
