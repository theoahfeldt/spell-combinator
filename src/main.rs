use bevy::prelude::*;
use spell_combinator::unit::{self, Position};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
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
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24., 24.), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let enemy_texture_handle = asset_server.load("textures/rpg/mobs/kobold-idle.png");
    let enemy_texture_atlas =
        TextureAtlas::from_grid(enemy_texture_handle, Vec2::new(24., 24.), 15, 1);
    let enemy_texture_atlas_handle = texture_atlases.add(enemy_texture_atlas);

    let enemy_positions = vec![
        (Position { x: -200., y: 150. }, Color::WHITE),
        (Position { x: 0., y: 120. }, Color::YELLOW),
        (Position { x: 220., y: 0. }, Color::TOMATO),
        (Position { x: -70., y: -350. }, Color::SALMON),
    ];

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Position { x: 0., y: 0. });

    enemy_positions.into_iter().for_each(|(pos, col)| {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: enemy_texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    color: col,
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::splat(2.)),
                ..Default::default()
            })
            .insert(Timer::from_seconds(0.1, true))
            .insert(pos);
    })
}
