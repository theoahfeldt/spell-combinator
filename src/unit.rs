use bevy::prelude::*;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn update_position_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.x, pos.y, 0.);
    }
}
