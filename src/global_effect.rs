use crate::{
    mouseclick::MouseClick,
    resources::{DefaultFont, TextureHandles},
    spell::Value,
    spellcircuit::{Output, SpellCircuit},
    unit::{self, Unit},
};
use bevy::prelude::*;

pub struct GlobalEffectPlugin;

impl Plugin for GlobalEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(execute_select_rubble)
            .add_system(execute_spawn_unit);
    }
}

#[derive(Component)]
pub struct SelectRubble {
    output: Output,
}

impl SelectRubble {
    pub fn new(output: Output) -> Self {
        Self { output }
    }
}

#[derive(Component)]
pub struct SpawnUnit {
    unit: Unit,
}

impl SpawnUnit {
    pub fn new(unit: Unit) -> Self {
        Self { unit }
    }
}

pub enum GlobalEffect {
    Select(SelectRubble),
    Spawn(SpawnUnit),
}

fn execute_select_rubble(
    mut commands: Commands,
    mut ev_mouseclick: EventReader<MouseClick>,
    mut q_circuit: Query<&mut SpellCircuit>,
    q_select: Query<(Entity, &SelectRubble)>,
) {
    if let Some((select_id, select)) = q_select.iter().next() {
        if let Some(click) = ev_mouseclick.iter().next() {
            let circuit = &mut q_circuit.single_mut();
            let rubble = commands.spawn().insert(click.world_position.clone()).id();
            if let Some(ref mut outputs) = circuit.nodes[select.output.node].outputs {
                outputs[select.output.index] = Value::Target(rubble);
            } else {
                panic!(
                    "Tried to replace output of unexecuted node {:?} with selected position.",
                    select.output.node
                );
            };
            commands.entity(select_id).despawn();
        }
    }
}

fn execute_spawn_unit(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    font: Res<DefaultFont>,
    query: Query<(Entity, &SpawnUnit)>,
) {
    for (id, spawn) in query.iter() {
        unit::spawn_unit(
            &mut commands,
            &texture_handles,
            font.0.clone(),
            spawn.unit.clone(),
        );
        commands.entity(id).despawn();
    }
}
