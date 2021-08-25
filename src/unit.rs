use crate::mouseclick::MouseClick;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct Target {
    pub center: Position,
    pub radius: f32,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum UnitType {
    Player,
    Kobold,
}

pub struct SpriteData {
    file: &'static str,
    dimensions: Vec2,
    columns: usize,
    rows: usize,
}

fn sprite_data(t: UnitType) -> SpriteData {
    match t {
        UnitType::Player => SpriteData {
            file: "textures/rpg/chars/gabe/gabe-idle-run.png",
            dimensions: Vec2::new(24., 24.),
            columns: 7,
            rows: 1,
        },
        UnitType::Kobold => SpriteData {
            file: "textures/rpg/mobs/kobold-idle.png",
            dimensions: Vec2::new(24., 24.),
            columns: 15,
            rows: 1,
        },
    }
}

#[derive(Clone)]
pub struct TextureHandles(HashMap<UnitType, Handle<TextureAtlas>>);

impl TextureHandles {
    pub fn new(
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> TextureHandles {
        let mut map = HashMap::new();
        for t in vec![UnitType::Player, UnitType::Kobold] {
            let sprite_data = sprite_data(t);
            let texture_handle = asset_server.load(sprite_data.file);
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                sprite_data.dimensions,
                sprite_data.columns,
                sprite_data.rows,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            map.insert(t, texture_atlas_handle);
        }
        TextureHandles(map)
    }
}

pub fn update_position_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.x, pos.y, 0.);
    }
}

pub fn spawn_unit(
    commands: &mut Commands,
    texture_handles: &TextureHandles,
    t: UnitType,
    pos: Position,
    col: Color,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_handles.0.get(&t).unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(pos.x, pos.y, 0.),
                scale: Vec3::splat(2.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                color: col,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(pos);
}

pub fn setup_units(mut commands: Commands, texture_handles: TextureHandles) {
    let enemy_positions = vec![
        (Position { x: -200., y: 160. }, Color::WHITE),
        (Position { x: 0., y: 120. }, Color::YELLOW),
        (Position { x: 220., y: 20. }, Color::CYAN),
        (Position { x: -70., y: -180. }, Color::ORANGE),
        (Position { x: -145., y: 280. }, Color::GREEN),
    ];

    spawn_unit(
        &mut commands,
        &texture_handles,
        UnitType::Player,
        Position { x: 0., y: 0. },
        Color::WHITE,
    );
    enemy_positions.into_iter().for_each(|(pos, col)| {
        spawn_unit(&mut commands, &texture_handles, UnitType::Kobold, pos, col)
    })
}

pub fn spawn_on_click_system(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    mut ev_mouseclick: EventReader<MouseClick>,
) {
    for click in ev_mouseclick.iter() {
        spawn_unit(
            &mut commands,
            &texture_handles,
            UnitType::Kobold,
            Position {
                x: click.0.x,
                y: click.0.y,
            },
            Color::WHITE,
        )
    }
}
