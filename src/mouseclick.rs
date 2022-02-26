use bevy::prelude::*;

use crate::types::Position;

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug)]
pub struct MouseClick {
    pub window_position: Vec2,
    pub world_position: Position,
}

pub fn mouse_button_debug_system(mut ev_mouseclick: EventReader<MouseClick>) {
    for click in ev_mouseclick.iter() {
        println!("Received mouse click event at: {:?}", click)
    }
}

pub fn mouse_button_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut ev_mouseclick: EventWriter<MouseClick>,
) {
    let win = windows.get_primary().unwrap();
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(pos) = win.cursor_position() {
            let world_coords = compute_world_coords(pos, win, q_camera.single());
            ev_mouseclick.send(MouseClick {
                window_position: pos,
                world_position: Position(world_coords),
            });
        }
    }
}

fn compute_world_coords(ui_position: Vec2, win: &Window, camera_transform: &Transform) -> Vec2 {
    let size = Vec2::new(win.width() as f32, win.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = ui_position - size / 2.0;
    // apply the camera transform
    let world_coords = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
    Vec2::new(world_coords.x, world_coords.y)
}
