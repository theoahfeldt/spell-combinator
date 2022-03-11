use crate::{
    resources::DefaultFont,
    spell::Spell,
    spellcircuit::{Active, CircuitNode, Output, SpellCircuit},
};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use rand::prelude::*;

const DEFAULT_OUTPUT_COLOR: Color = Color::BLACK;
const SELECTED_OUTPUT_COLOR: Color = Color::SILVER;

struct SpellCard {
    position: Vec2,
    name: String,
    description: String,
    node: usize,
}

#[derive(Clone)]
pub enum SpellInput {
    Spell(Output),
    CircuitOutput,
}

impl SpellInput {
    pub fn new(node: usize, index: usize) -> Self {
        SpellInput::Spell(Output::new(node, index))
    }
}

#[derive(Component)]
struct CardInput(SpellInput);

#[derive(Component)]
struct CardOutput(Output);

impl SpellCard {
    fn spawn(
        self,
        commands: &mut Commands,
        font: Handle<Font>,
        root: Entity,
        builder: &CircuitBuilder,
    ) {
        let Self {
            position,
            name,
            description,
            node,
        } = self;
        let child = commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    position: Rect {
                        left: Val::Px(position.x),
                        bottom: Val::Px(position.y),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(160.), Val::Px(160.)),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                color: Color::rgb(0.6, 0.7, 0.2).into(),
                ..Default::default()
            })
            .insert(SpellCardTag)
            .with_children(|parent| {
                // Input buttons
                parent
                    .spawn_bundle(NodeBundle {
                        node: Node {
                            size: Vec2::new(0., 0.),
                        },
                        style: Style {
                            size: Size::new(Val::Percent(20.), Val::Percent(100.)),
                            flex_direction: FlexDirection::ColumnReverse,
                            justify_content: JustifyContent::SpaceAround,
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    })
                    .insert(FocusPolicy::Pass)
                    .with_children(|parent| {
                        for input in 0..builder.nodes[node].spell.num_inputs {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(20.), Val::Px(20.)),
                                        ..Default::default()
                                    },
                                    color: DEFAULT_OUTPUT_COLOR.into(),
                                    ..Default::default()
                                })
                                .insert(CardInput(SpellInput::new(node, input)));
                        }
                    });
                // Text
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(60.), Val::Percent(100.)),
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    })
                    .insert(FocusPolicy::Pass)
                    .with_children(|parent| {
                        // Title
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                name,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 20.,
                                    color: Color::rgb(0., 0., 0.),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                        // Description
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                description,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 16.,
                                    color: Color::rgb(0., 0., 0.),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
                // Output buttons
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(20.), Val::Percent(100.)),
                            flex_direction: FlexDirection::ColumnReverse,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::FlexEnd,

                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    })
                    .insert(FocusPolicy::Pass)
                    .with_children(|parent| {
                        for output in 0..builder.nodes[node].spell.num_outputs {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(20.), Val::Px(20.)),
                                        ..Default::default()
                                    },
                                    color: DEFAULT_OUTPUT_COLOR.into(),
                                    ..Default::default()
                                })
                                .insert(CardOutput(Output::new(node, output)));
                        }
                    });
            })
            .id();
        commands.entity(root).push_children(&[child]);
    }
}

#[derive(Component)]
struct Selected(Vec2);

#[derive(Component)]
struct SpellCardTag;

fn select_card_system(
    mut commands: Commands,
    windows: Res<Windows>,
    interaction_query: Query<
        (Entity, &Style, &Interaction),
        (Changed<Interaction>, With<SpellCardTag>),
    >,
) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        for (card, style, interaction) in interaction_query.iter() {
            if let Interaction::Clicked = interaction {
                if let (Val::Px(x), Val::Px(y)) = (style.position.left, style.position.bottom) {
                    commands
                        .entity(card)
                        .insert(Selected(Vec2::new(x - position.x, y - position.y)));
                }
            } else {
                commands.entity(card).remove::<Selected>();
            }
        }
    }
}

fn move_card_system(
    mut cursor_evr: EventReader<CursorMoved>,
    mut card_query: Query<(&mut Style, &Selected), With<SpellCardTag>>,
) {
    for (mut style, Selected(pos)) in card_query.iter_mut() {
        for ev in cursor_evr.iter() {
            style.position.left = Val::Px(ev.position.x + pos.x);
            style.position.bottom = Val::Px(ev.position.y + pos.y)
        }
    }
}

fn select_output(
    mut commands: Commands,
    mut q_output: Query<
        (Entity, &Interaction, &mut UiColor),
        (With<CardOutput>, Changed<Interaction>, Without<Selected>),
    >,
    mut q_other: Query<(Entity, &mut UiColor), (With<CardOutput>, With<Selected>)>,
) {
    if let Ok((id, interaction, mut color)) = q_output.get_single_mut() {
        if let Interaction::Clicked = interaction {
            color.0 = SELECTED_OUTPUT_COLOR;
            commands.entity(id).insert(Selected(Vec2::ZERO));
            for (id, mut color) in q_other.iter_mut() {
                color.0 = DEFAULT_OUTPUT_COLOR;
                commands.entity(id).remove::<Selected>();
            }
        }
    }
}

fn connect_to_input(
    mut commands: Commands,
    mut builder: ResMut<CircuitBuilder>,
    mut q_input: Query<
        (&CardInput, &Interaction, &mut UiColor),
        (With<CardInput>, Changed<Interaction>),
    >,
    mut q_output: Query<(Entity, &CardOutput, &mut UiColor), (With<Selected>, Without<CardInput>)>,
    mut rng: ResMut<StdRng>,
) {
    if let Ok((input, interaction, mut i_color)) = q_input.get_single_mut() {
        if let Interaction::Clicked = interaction {
            if let Ok((output_id, output, mut o_color)) = q_output.get_single_mut() {
                builder.connect_io(input.0.clone(), output.0.clone());
                let color = Color::hsl(rng.gen::<f32>() * 360., 1., 0.5);
                i_color.0 = color;
                o_color.0 = color;
                commands.entity(output_id).remove::<Selected>();
            }
        }
    }
}

fn compile_circuit(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    builder: ResMut<CircuitBuilder>,
    q_active: Query<&Active>,
) {
    if keys.just_pressed(KeyCode::Return) && q_active.is_empty() {
        if let Some(circuit) = builder.compile() {
            commands.spawn_bundle((circuit, Active));
        }
    }
}

pub struct SpellBuilderPlugin;

impl Plugin for SpellBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(select_card_system)
            .add_system(move_card_system)
            .add_system(select_output)
            .add_system(connect_to_input)
            .add_system(compile_circuit);
    }
}

#[derive(Component)]
pub struct SpellBuilderUI;

fn setup(mut commands: Commands, font: Res<DefaultFont>) {
    let builder =
        CircuitBuilder::from_spells(vec![Spell::player(), Spell::punch(), Spell::constrict()]);
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::None,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.5).into(),
            ..Default::default()
        })
        .insert(SpellBuilderUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        position: Rect {
                            left: Val::Px(10.),
                            bottom: Val::Px(10.),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(30.), Val::Px(30.)),
                        ..Default::default()
                    },
                    color: DEFAULT_OUTPUT_COLOR.into(),
                    ..Default::default()
                })
                .insert(CardInput(SpellInput::CircuitOutput));
        })
        .id();
    let cards = (0..builder.nodes.len()).map(|i| SpellCard {
        position: Vec2::new(50.0 + 50.0 * i as f32, 50.0),
        name: "Test".to_string(),
        description: i.to_string(),
        node: i,
    });
    cards.for_each(|card| card.spawn(&mut commands, font.0.clone(), root, &builder));
    commands.insert_resource(builder);
}

struct BuilderNode {
    inputs: Vec<Option<Output>>,
    spell: Spell,
}

pub struct CircuitBuilder {
    nodes: Vec<BuilderNode>,
    output: Option<Output>,
}

impl CircuitBuilder {
    pub fn from_spells(spells: Vec<Spell>) -> Self {
        let nodes = spells
            .into_iter()
            .map(|spell| {
                let inputs = vec![None; spell.num_inputs];
                BuilderNode { inputs, spell }
            })
            .collect::<Vec<BuilderNode>>();
        Self {
            nodes,
            output: None,
        }
    }

    fn check_input(&self, input: &SpellInput) {
        if let SpellInput::Spell(ref input) = input {
            if input.node >= self.nodes.len() {
                panic!(
                    "Invalid input: {:?}. Circuit only has {:?} nodes.",
                    input,
                    self.nodes.len()
                )
            } else {
                let node = &self.nodes[input.node];
                if input.index >= node.spell.num_inputs {
                    panic!(
                        "Invalid output: {:?}. Node only has {:?} inputs.",
                        input, node.spell.num_inputs
                    )
                }
            }
        }
    }

    fn check_output(&self, output: &Output) {
        if output.node >= self.nodes.len() {
            panic!(
                "Invalid output: {:?}. Circuit only has {:?} nodes.",
                output,
                self.nodes.len()
            )
        } else {
            let node = &self.nodes[output.node];
            if output.index >= node.spell.num_outputs {
                panic!(
                    "Invalid output: {:?}. Node only has {:?} outputs.",
                    output, node.spell.num_outputs
                )
            }
        }
    }

    pub fn connect_io(&mut self, input: SpellInput, output: Output) {
        self.check_input(&input);
        match input {
            SpellInput::Spell(input) => {
                self.check_output(&output);
                let node = &mut self.nodes[input.node];
                node.inputs[input.index] = Some(output);
                // TODO: Check types and that no cycle is formed. Return bool based on this check.
            }
            SpellInput::CircuitOutput => {
                self.output = Some(output);
            }
        }
    }

    fn convert_node(node: &BuilderNode) -> Option<CircuitNode> {
        let inputs = node
            .inputs
            .clone()
            .into_iter()
            .collect::<Option<Vec<Output>>>();
        inputs.map(|inputs| CircuitNode::new(inputs, node.spell.clone()))
    }

    pub fn compile(&self) -> Option<SpellCircuit> {
        self.output.clone().and_then(|output| {
            let mut res: Option<Vec<CircuitNode>> = Some(vec![]);
            self.nodes.iter().for_each(|node| {
                res.as_mut()
                    .map(|res| CircuitBuilder::convert_node(node).map(|node| res.push(node)));
            });
            res.map(|nodes| SpellCircuit::new(nodes, output))
        })
    }
}
