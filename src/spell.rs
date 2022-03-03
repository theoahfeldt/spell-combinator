use crate::{
    effect::{Damage, Effect},
    types::{Health, Position},
};
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum Value {
    Target(Entity),
    Power(u32),
    Empty,
}

pub struct Unit {
    pub entity: Entity,
    pub health: Health,
    pub position: Position,
}

pub struct State {
    pub player: Unit,
    pub enemies: Vec<Unit>,
}

pub struct Spell {
    pub num_inputs: usize,
    pub num_outputs: usize,
    pub function: fn(&State, Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>),
}

impl Spell {
    pub fn new(
        num_inputs: usize,
        num_outputs: usize,
        function: fn(&State, Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>),
    ) -> Self {
        Self {
            num_inputs,
            num_outputs,
            function,
        }
    }
}

fn player(s: &State, _inputs: Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>) {
    let outputs = vec![Value::Target(s.player.entity)];
    let effects = vec![];
    (outputs, effects)
}

fn punch(_s: &State, target: Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>) {
    if let Value::Target(entity) = target[0] {
        let outputs = vec![Value::Empty];
        let effects = vec![(entity, Effect::Damage(Damage::new(69)))];
        (outputs, effects)
    } else {
        panic!("Bad input to punch {:?}", target);
    }
}

impl Spell {
    pub fn player() -> Self {
        Self::new(0, 1, player)
    }

    pub fn punch() -> Self {
        Self::new(1, 1, punch)
    }
}
