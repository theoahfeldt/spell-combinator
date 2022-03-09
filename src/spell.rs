use crate::{
    effect::{Damage, Effect},
    global_effect::{GlobalEffect, SelectRubble, SpawnUnit},
    spellcircuit::Output,
    types::{Health, Position, UnitType},
    unit::Unit,
};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Target(Entity),
    Power(u32),
    Empty,
}

#[derive(Clone)]
pub struct UnitInfo {
    pub health: Option<i32>,
    pub position: Vec2,
}

#[derive(Clone)]
pub struct SpellState {
    pub output: Output,
    pub player: Entity,
    pub units: HashMap<Entity, UnitInfo>,
}

type SpellResult = (Vec<Value>, Vec<(Entity, Effect)>, Vec<GlobalEffect>);

#[derive(Clone)]
pub struct Spell {
    pub num_inputs: usize,
    pub num_outputs: usize,
    pub function: fn(&SpellState, Vec<Value>) -> SpellResult,
}

impl Spell {
    pub fn new(
        num_inputs: usize,
        num_outputs: usize,
        function: fn(&SpellState, Vec<Value>) -> SpellResult,
    ) -> Self {
        Self {
            num_inputs,
            num_outputs,
            function,
        }
    }
}

fn player(s: &SpellState, _inputs: Vec<Value>) -> SpellResult {
    let outputs = vec![Value::Target(s.player)];
    let effects = vec![];
    let globals = vec![];
    (outputs, effects, globals)
}

fn punch(_s: &SpellState, inputs: Vec<Value>) -> SpellResult {
    if let Value::Target(entity) = inputs[0] {
        let outputs = vec![Value::Empty];
        let effects = vec![(entity, Effect::Damage(Damage::new(69)))];
        let globals = vec![];
        (outputs, effects, globals)
    } else {
        panic!("Bad inputs to punch: {:?}", inputs);
    }
}

fn introspection(s: &SpellState, _inputs: Vec<Value>) -> SpellResult {
    let outputs = vec![Value::Target(s.player), Value::Power(2)];
    let effects = vec![];
    let globals = vec![];
    (outputs, effects, globals)
}

fn air(_s: &SpellState, _inputs: Vec<Value>) -> SpellResult {
    let outputs = vec![Value::Power(0)];
    let effects = vec![];
    let globals = vec![];
    (outputs, effects, globals)
}

fn constrict(_s: &SpellState, inputs: Vec<Value>) -> SpellResult {
    if let Value::Target(entity) = inputs[0] {
        let outputs = vec![Value::Target(entity)];
        let effects = vec![(entity, Effect::Damage(Damage::new(3)))];
        let globals = vec![];
        (outputs, effects, globals)
    } else {
        panic!("Bad inputs to constrict: {:?}", inputs);
    }
}

fn draw_life(s: &SpellState, inputs: Vec<Value>) -> SpellResult {
    if let Value::Target(entity) = inputs[0] {
        let damage = s.units.get(&entity).unwrap().health.unwrap_or(0) / 10;
        let damage = if damage >= 0 { damage } else { 0 };
        let outputs = vec![Value::Power(damage as u32)];
        let effects = vec![(entity, Effect::Damage(Damage::new(damage)))];
        let globals = vec![];
        (outputs, effects, globals)
    } else {
        panic!("Bad inputs to draw life: {:?}", inputs);
    }
}

fn scout(s: &SpellState, _inputs: Vec<Value>) -> SpellResult {
    let outputs = vec![Value::Empty];
    let effects = vec![];
    let globals = vec![GlobalEffect::Select(SelectRubble::new(s.output.clone()))];
    (outputs, effects, globals)
}

fn spawn_cobold(s: &SpellState, inputs: Vec<Value>) -> SpellResult {
    if let Value::Target(entity) = inputs[0] {
        let position = s
            .units
            .get(&entity)
            .expect("Target of spawn cobold does not exist!")
            .position;
        let cobold = Unit {
            health: Health(10),
            position: Position(position),
            unit_type: UnitType::Kobold,
        };
        let outputs = vec![Value::Empty];
        let effects = vec![];
        let globals = vec![GlobalEffect::Spawn(SpawnUnit::new(cobold))];
        (outputs, effects, globals)
    } else {
        panic!("Bad inputs to spawn cobold: {:?}", inputs);
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

    pub fn scout() -> Self {
        Self::new(0, 1, scout)
    }

    pub fn spawn_cobold() -> Self {
        Self::new(1, 1, spawn_cobold)
    }
}
