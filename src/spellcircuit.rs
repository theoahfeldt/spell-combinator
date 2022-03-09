use crate::{
    effect::{Effect, Effects},
    global_effect::{GlobalEffect, SelectRubble, SpawnUnit},
    spell::{Spell, SpellState, UnitInfo, Value},
    types::{Health, Position},
    unit::Player,
};
use bevy::prelude::*;
use std::collections::HashMap;

// Pointer to a specific output of a specific node in a spellcircuit
#[derive(Clone, Debug)]
pub struct Output {
    pub node: usize,
    pub index: usize,
}

impl Output {
    pub fn new(node: usize, index: usize) -> Self {
        Self { node, index }
    }
}

#[derive(Clone)]
pub struct CircuitNode {
    inputs: Vec<Output>,
    pub outputs: Option<Vec<Value>>,
    spell: Spell,
}

impl CircuitNode {
    pub fn new(inputs: Vec<Output>, spell: Spell) -> Self {
        Self {
            inputs,
            outputs: None,
            spell,
        }
    }
}

pub struct SpellCircuit {
    pub nodes: Vec<CircuitNode>,
    output: Output,
}

impl SpellCircuit {
    pub fn new(nodes: Vec<CircuitNode>, output: Output) -> Self {
        Self { nodes, output }
    }

    pub fn execute_next_spell(
        &mut self,
        s: &SpellState,
    ) -> Option<(Vec<(Entity, Effect)>, Vec<GlobalEffect>)> {
        self.execute_next_spell_rec(s, &self.output.clone())
    }

    fn execute_next_spell_rec(
        &mut self,
        s: &SpellState,
        output: &Output,
    ) -> Option<(Vec<(Entity, Effect)>, Vec<GlobalEffect>)> {
        if let Some(_) = self.nodes[output.node].outputs {
            None
        } else {
            self.nodes[output.node]
                .inputs
                .clone()
                .into_iter()
                .fold(None, |acc, o| {
                    acc.or_else(|| self.execute_next_spell_rec(s, &o))
                })
                .or_else(|| {
                    let inputs = self.nodes[output.node]
                        .inputs
                        .iter()
                        .map(|o| self.nodes[o.node].outputs.clone().unwrap()[o.index].clone())
                        .collect();
                    let state = &SpellState {
                        output: output.clone(),
                        ..s.clone()
                    };
                    let res = (self.nodes[output.node].spell.function)(state, inputs);
                    self.nodes[output.node].outputs = Some(res.0.clone());
                    Some((res.1, res.2))
                })
        }
    }
}

pub fn example_circuit() -> SpellCircuit {
    let scout = CircuitNode::new(vec![], Spell::scout());
    let spawn = CircuitNode::new(vec![Output::new(0, 0)], Spell::spawn_cobold());
    let nodes = vec![scout, spawn];
    let output = Output::new(1, 0);
    SpellCircuit { nodes, output }
}

#[derive(Component)]
pub struct ExampleCircuit(pub SpellCircuit);

pub struct CircuitPlugin;

struct EffectsDone(bool);

impl Plugin for CircuitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::PreUpdate, wait_for_effects)
            .add_system(execute_spell_circuit_system);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(EffectsDone(true));
    commands.spawn().insert(ExampleCircuit(example_circuit()));
}

fn wait_for_effects(
    mut effects: ResMut<EffectsDone>,
    q_units: Query<&Effects>,
    q_global: Query<Entity, Or<(With<SelectRubble>, With<SpawnUnit>)>>,
) {
    if q_units.iter().find(|x| !x.0.is_empty()).is_none() && q_global.is_empty() {
        effects.0 = true;
    } else {
        effects.0 = false;
    }
}

fn execute_spell_circuit_system(
    mut commands: Commands,
    effects: ResMut<EffectsDone>,
    mut q_circuit: Query<&mut ExampleCircuit>,
    mut q_units: Query<(Entity, &Health, &Position, &mut Effects)>,
    q_rubble: Query<(Entity, &Position), Without<Health>>,
    q_player: Query<Entity, With<Player>>,
) {
    if effects.0 {
        let circuit = &mut q_circuit.single_mut().0;
        let mut units: HashMap<Entity, UnitInfo> = q_units
            .iter()
            .map(|(entity, health, pos, _)| {
                (
                    entity,
                    UnitInfo {
                        health: Some(health.0),
                        position: pos.0,
                    },
                )
            })
            .collect();
        for (entity, pos) in q_rubble.iter() {
            units.insert(
                entity,
                UnitInfo {
                    health: None,
                    position: pos.0,
                },
            );
        }
        let player = q_player.single();
        if let Some((new_effects, new_globals)) = circuit.execute_next_spell(&SpellState {
            output: Output::new(0, 0),
            player,
            units,
        }) {
            for (entity, effect) in new_effects.into_iter() {
                let entry = &mut q_units.get_mut(entity).unwrap();
                entry.3 .0.push_back(effect);
            }
            for effect in new_globals.into_iter() {
                match effect {
                    GlobalEffect::Select(select) => {
                        commands.spawn().insert(select);
                    }
                    GlobalEffect::Spawn(spawn) => {
                        commands.spawn().insert(spawn);
                    }
                }
            }
        }
    }
}
