use std::collections::VecDeque;

use crate::types::{Health, Position};
use bevy::prelude::*;

#[derive(Component)]
pub struct Move {
    target: Position,
}

impl Move {
    pub fn new(target: Position) -> Self {
        Self { target }
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
        self.target.clone().map(|t| Move::new(t))
    }
}

pub struct Damage {
    damage: i32,
}

impl Damage {
    pub fn new(damage: i32) -> Self {
        Self { damage }
    }

    pub fn update(&self, health: &mut Health) -> bool {
        health.0 -= self.damage;
        true
    }
}

pub enum Effect {
    Move(Move),
    Damage(Damage),
}

impl Effect {
    pub fn update(&self, health: &mut Health, position: &mut Position) -> bool {
        match self {
            Effect::Move(m) => m.update(position),
            Effect::Damage(d) => d.update(health),
        }
    }
}

#[derive(Component)]
pub struct Effects(pub VecDeque<Effect>);

impl Effects {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }
}
