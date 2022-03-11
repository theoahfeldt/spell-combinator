use crate::types::UnitType;
use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

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
pub struct TextureHandles(pub HashMap<UnitType, Handle<TextureAtlas>>);

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

pub struct DefaultFont(pub Handle<Font>);

impl DefaultFont {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self(asset_server.load("fonts/FiraSans-Bold.ttf"))
    }
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup)
            .add_system(animate_sprite_system);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handles = TextureHandles::new(&asset_server, &mut texture_atlases);
    commands.insert_resource(texture_handles);
    commands.insert_resource(DefaultFont::new(&asset_server));
    commands.insert_resource(StdRng::from_entropy());
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
        }
    }
}
