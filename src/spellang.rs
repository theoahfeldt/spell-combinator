use crate::{
    effect::{Damage, Effect, Effects, Move},
    types::{Health, Position},
    unit::{Player, Unit},
};
use bevy::prelude::*;

pub struct State {
    pub player: Unit,
    pub enemies: Vec<Unit>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Target(Entity),
    Empty,
}

pub struct Spell {
    num_inputs: usize,
    num_outputs: usize,
    outputs: Option<Vec<Value>>,
    function: fn(&State, Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>),
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
            outputs: None,
            function,
        }
    }

    fn get_outputs(&mut self, s: &State, inputs: Vec<Value>) -> Vec<Value> {
        match &self.outputs {
            Some(x) => x.clone(),
            None => {
                let x = (self.function)(s, inputs).0;
                self.outputs = Some(x.clone());
                x
            }
        }
    }
}

#[derive(Clone)]
pub struct Node {
    inputs: Vec<Output>,
    spell_idx: usize,
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
        self.execute_next_spell_rec(s, self.nodes[self.output.node].clone())
    }

    fn execute_next_spell_rec(&mut self, s: &State, node: Node) -> Option<Vec<(Entity, Effect)>> {
        if let Some(_) = self.spells[node.spell_idx].outputs {
            None
        } else {
            node.inputs
                .clone()
                .into_iter()
                .fold(None, |acc, o| {
                    acc.or_else(|| self.execute_next_spell_rec(s, self.nodes[o.node].clone()))
                })
                .or_else(|| {
                    let inputs = node
                        .inputs
                        .iter()
                        .map(|o| {
                            let spell_idx = self.nodes[o.node].spell_idx;
                            let spell = &self.spells[spell_idx];
                            spell.outputs.clone().unwrap()[o.index].clone()
                        })
                        .collect();
                    let spell = &mut self.spells[node.spell_idx];
                    let res = (spell.function)(s, inputs);
                    spell.outputs = Some(res.0.clone());
                    Some(res.1)
                })
        }
    }

    pub fn execute(&mut self, s: &State) -> Result<Value, String> {
        self.execute_rec(s, self.output.clone())
    }

    fn execute_rec(&mut self, s: &State, output: Output) -> Result<Value, String> {
        let node = self.nodes[output.node].clone();
        if output.index >= self.spells[node.spell_idx].num_outputs {
            Err(format!(
                "Attempted to extract non-existent output: {:?}",
                output
            ))
        } else {
            if let Some(res) = self.spells[node.spell_idx].outputs.clone() {
                Ok(res[output.index].clone())
            } else {
                let outputs: Result<Vec<Value>, String> = node
                    .inputs
                    .iter()
                    .map(|o| self.execute_rec(s, o.clone()))
                    .into_iter()
                    .collect();
                outputs.and_then(|inputs| {
                    if inputs.len() < self.spells[node.spell_idx].num_inputs {
                        Err(format!("Not enough inputs for spell: {:?}", node.spell_idx))
                    } else {
                        Ok(self
                            .spells
                            .get_mut(node.spell_idx)
                            .unwrap()
                            .get_outputs(s, inputs)[output.index]
                            .clone())
                    }
                })
            }
        }
    }
}

fn player(s: &State, _inputs: Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>) {
    (vec![Value::Target(s.player.entity)], vec![])
}

fn _punch(_s: &State, target: Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>) {
    if let Value::Target(entity) = target[0] {
        (
            vec![Value::Empty],
            vec![(entity, Effect::Damage(Damage::new(69)))],
        )
    } else {
        panic!("Bad input to punch {:?}", target);
    }
}

fn move_to_42(_s: &State, target: Vec<Value>) -> (Vec<Value>, Vec<(Entity, Effect)>) {
    if let Value::Target(entity) = target[0] {
        (
            vec![Value::Empty],
            vec![(
                entity,
                Effect::Move(Move::new(Position(Vec2::new(42., 42.)))),
            )],
        )
    } else {
        panic!("Bad input to punch {:?}", target);
    }
}

pub fn example_circuit() -> SpellCircuit {
    let player = Spell::new(0, 1, player);
    let punch = Spell::new(1, 1, move_to_42);
    let spells = vec![player, punch];
    let player_node = Node {
        inputs: vec![],
        spell_idx: 0,
    };
    let punch_node = Node {
        inputs: vec![Output::new(0, 0)],
        spell_idx: 1,
    };
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
