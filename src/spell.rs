use crate::effect::{Damage, Effect};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Target(Entity),
    Power(u32),
    Empty,
}

pub struct Unit {
    pub health: i32,
    pub position: Vec2,
}

pub struct State {
    pub player: Entity,
    pub units: HashMap<Entity, Unit>,
}

type SpellResult = (Vec<Value>, Vec<(Entity, Effect)>);

pub struct Spell {
    pub num_inputs: usize,
    pub num_outputs: usize,
    pub function: fn(&State, Vec<Value>) -> SpellResult,
}

impl Spell {
    pub fn new(
        num_inputs: usize,
        num_outputs: usize,
        function: fn(&State, Vec<Value>) -> SpellResult,
    ) -> Self {
        Self {
            num_inputs,
            num_outputs,
            function,
        }
    }
}

fn player(s: &State, _inputs: Vec<Value>) -> SpellResult {
    let outputs = vec![Value::Target(s.player)];
    let effects = vec![];
    (outputs, effects)
}

fn punch(_s: &State, inputs: Vec<Value>) -> SpellResult {
    if let Value::Target(entity) = inputs[0] {
        let outputs = vec![Value::Empty];
        let effects = vec![(entity, Effect::Damage(Damage::new(69)))];
        (outputs, effects)
    } else {
        panic!("Bad inputs to punch: {:?}", inputs);
    }
}

fn introspection(s: &State, _inputs: Vec<Value>) -> SpellResult {
    let outputs = vec![Value::Target(s.player), Value::Power(2)];
    let effects = vec![];
    (outputs, effects)
}

fn air(_s: &State, _inputs: Vec<Value>) -> SpellResult {
    let outputs = vec![Value::Power(0)];
    let effects = vec![];
    (outputs, effects)
}

fn constrict(_s: &State, inputs: Vec<Value>) -> SpellResult {
    if let Value::Target(entity) = inputs[0] {
        let outputs = vec![Value::Target(entity)];
        let effects = vec![(entity, Effect::Damage(Damage::new(3)))];
        (outputs, effects)
    } else {
        panic!("Bad inputs to constrict: {:?}", inputs);
    }
}

fn draw_life(s: &State, inputs: Vec<Value>) -> SpellResult {
    if let Value::Target(entity) = inputs[0] {
        let damage = s.units.get(&entity).unwrap().health / 10;
        let damage = if damage >= 0 { damage } else { 0 };
        let outputs = vec![Value::Power(damage as u32)];
        let effects = vec![(entity, Effect::Damage(Damage::new(damage)))];
        (outputs, effects)
    } else {
        panic!("Bad inputs to draw life: {:?}", inputs);
    }
}

impl Spell {
    pub fn player() -> Self {
        Self::new(0, 1, player)
    }

    pub fn punch() -> Self {
        Self::new(1, 1, punch)
    }

    pub fn introspection() -> Self {
        Self::new(0, 1, introspection)
    }

    pub fn air() -> Self {
        Self::new(0, 1, air)
    }

    pub fn constrict() -> Self {
        Self::new(1, 1, constrict)
    }

    pub fn draw_life() -> Self {
        Self::new(1, 1, draw_life)
    }
}
