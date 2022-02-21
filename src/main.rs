use bevy::prelude::*;
use spell_combinator::effect::MovePlugin;
use spell_combinator::mouseclick::{self, MainCamera, MouseClick};
use spell_combinator::unit::{TextureHandles, UnitPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MouseClick>()
        .add_startup_system(setup)
        .add_plugin(UnitPlugin)
        .add_plugin(MovePlugin)
        .add_system(mouseclick::mouse_button_system.label("mouseclick"))
        .add_system(animate_sprite_system)
        .run();
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handles = TextureHandles::new(&asset_server, &mut texture_atlases);
    commands.insert_resource(texture_handles);
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
