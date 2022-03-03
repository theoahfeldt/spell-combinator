use crate::{
    effect::{Effect, Effects},
    spell::{Spell, State, Unit, Value},
    types::{Health, Position},
    unit::Player,
};
use bevy::prelude::*;

#[derive(Clone)]
pub struct Node {
    inputs: Vec<Output>,
    outputs: Option<Vec<Value>>,
    spell_idx: usize,
}

impl Node {
    pub fn new(inputs: Vec<Output>, spell_idx: usize) -> Self {
        Self {
            inputs,
            outputs: None,
            spell_idx,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Output {
    node: usize,
    index: usize,
}

impl Output {
    pub fn new(node: usize, index: usize) -> Self {
        Self { node, index }
    }
}

pub struct SpellCircuit {
    nodes: Vec<Node>,
    output: Output,
    spells: Vec<Spell>,
}

impl SpellCircuit {
    pub fn execute_next_spell(&mut self, s: &State) -> Option<Vec<(Entity, Effect)>> {
        self.execute_next_spell_rec(s, self.output.node)
    }

    fn execute_next_spell_rec(
        &mut self,
        s: &State,
        node_idx: usize,
    ) -> Option<Vec<(Entity, Effect)>> {
        if let Some(_) = self.nodes[node_idx].outputs {
            None
        } else {
            self.nodes[node_idx]
                .inputs
                .clone()
                .into_iter()
                .fold(None, |acc, o| {
                    acc.or_else(|| self.execute_next_spell_rec(s, o.node))
                })
                .or_else(|| {
                    let inputs = self.nodes[node_idx]
                        .inputs
                        .iter()
                        .map(|o| self.nodes[o.node].outputs.clone().unwrap()[o.index].clone())
                        .collect();
                    let res = (self.spells[self.nodes[node_idx].spell_idx].function)(s, inputs);
                    self.nodes[node_idx].outputs = Some(res.0.clone());
                    Some(res.1)
                })
        }
    }
}

pub fn example_circuit() -> SpellCircuit {
    let spells = vec![Spell::player(), Spell::punch()];
    let player_node = Node::new(vec![], 0);
    let punch_node = Node::new(vec![Output::new(0, 0)], 1);
    let nodes = vec![player_node, punch_node];
    let output = Output::new(1, 0);
    SpellCircuit {
        nodes,
        output,
        spells,
    }
}

#[derive(Component)]
struct ExampleCircuit(SpellCircuit);

pub struct CircuitPlugin;

impl Plugin for CircuitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spell_circuit)
            .add_system(update_spell_circuit_system);
    }
}

fn setup_spell_circuit(mut commands: Commands) {
    commands.spawn().insert(ExampleCircuit(example_circuit()));
}

fn update_spell_circuit_system(
    mut circuit_query: Query<&mut ExampleCircuit>,
    mut enemy_query: Query<(Entity, &Health, &Position, &mut Effects), Without<Player>>,
    mut player_query: Query<(Entity, &Health, &Position, &mut Effects), With<Player>>,
) {
    let circuit = &mut circuit_query.iter_mut().next().unwrap().0;
    let player = (|x: (Entity, &Health, &Position, &Effects)| Unit {
        entity: x.0,
        health: x.1.clone(),
        position: x.2.clone(),
    })(player_query.single());
    let enemies = enemy_query
        .iter()
        .map(|(entity, health, position, _)| Unit {
            entity,
            health: health.clone(),
            position: position.clone(),
        })
        .collect();
    if let Some(new_effects) = circuit.execute_next_spell(&State { player, enemies }) {
        for (entity, effect) in new_effects.into_iter() {
            let entry = &mut enemy_query
                .get_mut(entity)
                .or(player_query.get_mut(entity))
                .unwrap();
            entry.3 .0.push_back(effect);
        }
    }
}