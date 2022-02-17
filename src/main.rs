use bevy::prelude::*;
use spell_combinator::mouseclick::{self, MainCamera, MouseClick};
use spell_combinator::unit::{self, TextureHandles};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<MouseClick>()
        .add_startup_system(setup.system())
        .add_system(mouseclick::mouse_button_system.system().label("mouseclick"))
        .add_system(unit::spawn_on_click_system.system().after("mouseclick"))
        .add_system(unit::update_position_system.system())
        .add_system(animate_sprite_system.system())
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
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handles = TextureHandles::new(&asset_server, &mut texture_atlases);
    commands.insert_resource(texture_handles.clone());
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    unit::setup_units(commands, texture_handles)
}