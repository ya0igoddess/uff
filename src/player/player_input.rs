use bevy::{ecs::system::{Query, Res}, input::{keyboard::KeyCode, ButtonInput}, time::Time, transform::components::Transform};

use super::player::InputControllable;

pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &InputControllable)>
) {
    let pos_delta = 100.0 * time.delta_seconds();
    for (mut transform, _) in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += pos_delta;
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= pos_delta;
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= pos_delta;
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += pos_delta;
        }
    }
}