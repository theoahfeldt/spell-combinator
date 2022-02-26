use bevy::prelude::*;
use spell_combinator::mouseclick::{self, MainCamera, MouseClick};
use spell_combinator::resources::ResourcePlugin;
use spell_combinator::spellang::CircuitPlugin;
use spell_combinator::spellbuilder::SpellBuilderPlugin;
use spell_combinator::ui::{ButtonClick, UiPlugin};
use spell_combinator::unit::UnitPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.6, 0.7)))
        .add_plugins(DefaultPlugins)
        .add_event::<MouseClick>()
        .add_event::<ButtonClick>()
        .add_startup_system(setup)
        .add_plugin(ResourcePlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(CircuitPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(SpellBuilderPlugin)
        .add_system(mouseclick::mouse_button_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
