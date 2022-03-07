use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Position(pub Vec2);

#[derive(Component, Clone, Debug)]
pub struct Health(pub i32);

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum UnitType {
    Player,
    Kobold,
}
