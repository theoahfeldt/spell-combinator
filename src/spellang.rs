use crate::spellang::{
    Node::{N1to1, N1to2, N2to1},
    Type::{Damage, Unit},
};

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub hp: i32,
}

#[derive(Clone, Debug)]
pub enum Type {
    Damage(i32),
    Unit,
}

pub struct Spell1to1 {
    output: Option<Type>,
    effect: fn(&mut State, Type) -> Type,
}

impl Spell1to1 {
    pub fn new(effect: fn(&mut State, Type) -> Type) -> Self {
        Self {
            output: None,
            effect,
        }
    }

    fn get_output(&mut self, s: &mut State, input: Type) -> Type {
        match &self.output {
            Some(x) => x.clone(),
            None => {
                let x = (self.effect)(s, input);
                self.output = Some(x.clone());
                x
            }
        }
    }
}

#[derive(Clone)]
pub struct Node1to1 {
    input: Option<Input>,
    spell_idx: usize,
}

pub struct Spell1to2 {
    output: Option<(Type, Type)>,
    effect: fn(&mut State, Type) -> (Type, Type),
}

impl Spell1to2 {
    pub fn new(effect: fn(&mut State, Type) -> (Type, Type)) -> Self {
        Self {
            output: None,
            effect,
        }
    }

    fn get_output(&mut self, s: &mut State, input: Type) -> (Type, Type) {
        match &self.output {
            Some(x) => x.clone(),
            None => {
                let x = (self.effect)(s, input);
                self.output = Some(x.clone());
                x
            }
        }
    }
}

#[derive(Clone)]
pub struct Node1to2 {
    input: Option<Input>,
    spell_idx: usize,
}

pub struct Spell2to1 {
    output: Option<Type>,
    effect: fn(&mut State, Type, Type) -> Type,
}

impl Spell2to1 {
    pub fn new(effect: fn(&mut State, Type, Type) -> Type) -> Self {
        Self {
            output: None,
            effect,
        }
    }

    fn get_output(&mut self, s: &mut State, input1: Type, input2: Type) -> Type {
        match &self.output {
            Some(x) => x.clone(),
            None => {
                let x = (self.effect)(s, input1, input2);
                self.output = Some(x.clone());
                x
            }
        }
    }
}

#[derive(Clone)]
pub struct Node2to1 {
    input1: Option<Input>,
    input2: Option<Input>,
    spell_idx: usize,
}

#[derive(Clone)]
pub enum OutputIndex {
    First,
    Second,
}

#[derive(Clone)]
pub struct Output {
    index: OutputIndex,
    node: usize,
}

#[derive(Clone)]
pub enum Input {
    Unit,
    Other(Output),
}

#[derive(Clone)]
pub enum Node {
    N1to1(Node1to1),
    N2to1(Node2to1),
    N1to2(Node1to2),
}

pub struct SpellDAG {
    nodes: Vec<Node>,
    output: Output,
    spells1to1: Vec<Spell1to1>,
    spells2to1: Vec<Spell2to1>,
    spells1to2: Vec<Spell1to2>,
}

impl SpellDAG {
    pub fn execute(&mut self, s: &mut State) -> Option<Type> {
        self.execute_rec(s, self.output.clone())
    }
    fn execute_rec(&mut self, s: &mut State, output: Output) -> Option<Type> {
        let node = self.nodes[output.node].clone();
        if let Some(res) = match node {
            N1to1(ref n) => self.spells1to1[n.spell_idx].output.clone(),
            N2to1(ref n) => self.spells2to1[n.spell_idx].output.clone(),
            N1to2(ref n) => {
                let os = self.spells1to2[n.spell_idx].output.clone();
                match output.index {
                    OutputIndex::First => os.map(|x| x.0),
                    OutputIndex::Second => os.map(|x| x.1),
                }
            }
        } {
            Some(res)
        } else {
            match node {
                N1to1(n) => n.input.clone().and_then(|input| {
                    let i = match input {
                        Input::Unit => Some(Type::Unit),
                        Input::Other(o) => self.execute_rec(s, o),
                    };
                    i.map(|x| {
                        self.spells1to1
                            .get_mut(n.spell_idx)
                            .unwrap()
                            .get_output(s, x)
                    })
                }),
                N2to1(n) => n.input1.clone().and_then(|input1| {
                    n.input2.clone().and_then(|input2| {
                        let i1 = match input1 {
                            Input::Unit => Some(Type::Unit),
                            Input::Other(o) => self.execute_rec(s, o),
                        };
                        let i2 = match input2 {
                            Input::Unit => Some(Type::Unit),
                            Input::Other(o) => self.execute_rec(s, o),
                        };
                        i1.and_then(|x| {
                            i2.map(|y| {
                                self.spells2to1
                                    .get_mut(n.spell_idx)
                                    .unwrap()
                                    .get_output(s, x, y)
                            })
                        })
                    })
                }),
                N1to2(n) => n.input.clone().and_then(|input| {
                    let i = match input {
                        Input::Unit => Some(Type::Unit),
                        Input::Other(o) => self.execute_rec(s, o),
                    };
                    i.map(|x| {
                        (|os: (Type, Type)| match output.index {
                            OutputIndex::First => os.0,
                            OutputIndex::Second => os.1,
                        })(
                            self.spells1to2
                                .get_mut(n.spell_idx)
                                .unwrap()
                                .get_output(s, x),
                        )
                    })
                }),
            }
        }
    }
}

// pub fn take_stance(_s: &mut State, _input: Type) -> Type {
//     Damage(0)
// }

pub fn dual_wield(_s: &mut State, _input: Type) -> (Type, Type) {
    (Damage(0), Damage(0))
}

pub fn combine(_s: &mut State, input1: Type, input2: Type) -> Type {
    if let (Damage(x), Damage(y)) = (input1, input2) {
        Damage(x + y)
    } else {
        Unit
    }
}

pub fn focus(_s: &mut State, damage: Type) -> Type {
    match damage {
        Damage(x) => Damage(x + 1),
        Unit => Unit,
    }
}

pub fn attack(s: &mut State, damage: Type) -> Type {
    if let Damage(x) = damage {
        s.hp -= x;
    };
    Unit
}

pub fn example_dag() -> SpellDAG {
    let dual = Spell1to2::new(dual_wield);
    let focus = Spell1to1::new(focus);
    let comb = Spell2to1::new(combine);
    let attack = Spell1to1::new(attack);
    let spells1to1 = vec![focus, attack];
    let spells2to1 = vec![comb];
    let spells1to2 = vec![dual];
    let dual_node = Node::N1to2(Node1to2 {
        input: Some(Input::Unit),
        spell_idx: 0,
    });
    let focus_node = Node::N1to1(Node1to1 {
        input: Some(Input::Other(Output {
            index: OutputIndex::First,
            node: 0,
        })),
        spell_idx: 0,
    });
    let comb_node = Node::N2to1(Node2to1 {
        input1: Some(Input::Other(Output {
            index: OutputIndex::First,
            node: 1,
        })),
        input2: Some(Input::Other(Output {
            index: OutputIndex::Second,
            node: 0,
        })),
        spell_idx: 0,
    });
    let attack_node = Node::N1to1(Node1to1 {
        input: Some(Input::Other(Output {
            index: OutputIndex::First,
            node: 2,
        })),
        spell_idx: 1,
    });
    let nodes = vec![dual_node, focus_node, comb_node, attack_node];
    let output = Output {
        index: OutputIndex::First,
        node: 3,
    };
    SpellDAG {
        nodes,
        output,
        spells1to1,
        spells2to1,
        spells1to2,
    }
}
