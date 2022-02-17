use crate::spellang::Type::Damage;

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub hp: i32,
}

#[derive(Clone, Debug)]
pub enum Type {
    Damage(i32),
}

pub struct Spell {
    num_inputs: usize,
    num_outputs: usize,
    outputs: Option<Vec<Type>>,
    effect: fn(&mut State, Vec<Type>) -> Vec<Type>,
}

impl Spell {
    pub fn new(
        num_inputs: usize,
        num_outputs: usize,
        effect: fn(&mut State, Vec<Type>) -> Vec<Type>,
    ) -> Self {
        Self {
            num_inputs,
            num_outputs,
            outputs: None,
            effect,
        }
    }

    fn get_outputs(&mut self, s: &mut State, inputs: Vec<Type>) -> Vec<Type> {
        match &self.outputs {
            Some(x) => x.clone(),
            None => {
                let x = (self.effect)(s, inputs);
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
    index: usize,
    node: usize,
}

impl Output {
    pub fn new(index: usize, node: usize) -> Self {
        Self { index, node }
    }
}

pub struct SpellCircuit {
    nodes: Vec<Node>,
    output: Output,
    spells: Vec<Spell>,
}

impl SpellCircuit {
    pub fn execute(&mut self, s: &mut State) -> Result<Type, String> {
        self.execute_rec(s, self.output.clone())
    }

    fn execute_rec(&mut self, s: &mut State, output: Output) -> Result<Type, String> {
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
                let outputs: Result<Vec<Type>, String> = node
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

// pub fn take_stance(_s: &mut State, _input: Type) -> Type {
//     Damage(0)
// }

pub fn dual_wield(_s: &mut State, _input: Vec<Type>) -> Vec<Type> {
    vec![Damage(0), Damage(0)]
}

pub fn combine(_s: &mut State, input: Vec<Type>) -> Vec<Type> {
    let (Damage(x), Damage(y)) = (input[0].clone(), input[1].clone());
    vec![Damage(x + y)]
}

pub fn focus(_s: &mut State, damage: Vec<Type>) -> Vec<Type> {
    match damage[0] {
        Damage(x) => vec![Damage(x + 1)],
    }
}

pub fn attack(s: &mut State, damage: Vec<Type>) -> Vec<Type> {
    let Damage(x) = damage[0];
    s.hp -= x;
    vec![Damage(0)]
}

pub fn example_dag() -> SpellCircuit {
    let dual = Spell::new(0, 2, dual_wield);
    let focus = Spell::new(1, 1, focus);
    let comb = Spell::new(2, 1, combine);
    let attack = Spell::new(1, 1, attack);
    let spells = vec![dual, focus, comb, attack];
    let dual_node = Node {
        inputs: vec![],
        spell_idx: 0,
    };
    let focus_node = Node {
        inputs: vec![Output::new(0, 0)],
        spell_idx: 1,
    };
    let comb_node = Node {
        inputs: vec![Output::new(1, 0), Output::new(0, 1)],
        spell_idx: 2,
    };
    let attack_node = Node {
        inputs: vec![Output::new(0, 2)],
        spell_idx: 3,
    };
    let nodes = vec![dual_node, focus_node, comb_node, attack_node];
    let output = Output::new(0, 3);
    SpellCircuit {
        nodes,
        output,
        spells,
    }
}
