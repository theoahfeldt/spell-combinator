use bevy::prelude::*;
use spell_combinator::mouseclick::{self, MainCamera, MouseClick};
use spell_combinator::resources::ResourcePlugin;
use spell_combinator::spellang::CircuitPlugin;
use spell_combinator::ui::{ButtonClick, UiPlugin};
use spell_combinator::unit::UnitPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MouseClick>()
        .add_event::<ButtonClick>()
        .add_startup_system(setup)
        .add_plugin(ResourcePlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(CircuitPlugin)
        .add_plugin(UiPlugin)
        .add_system(mouseclick::mouse_button_system.label("mouseclick"))
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
