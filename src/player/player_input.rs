use bevy::{ecs::system::{Query, Res}, input::{keyboard::KeyCode, ButtonInput}, time::Time, transform::components::Transform};
use bevy_rapier2d::dynamics::Velocity;

use super::player::InputControllable;

pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &InputControllable)>
) {
    let pos_delta = 3000.0 * time.delta_seconds();
    for (mut vel, _) in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            vel.linvel.y = pos_delta;
            vel.linvel.x = 0.;
        }
        if keys.pressed(KeyCode::KeyA) {
            vel.linvel.x = -pos_delta;
            vel.linvel.y = 0.;
        }
        if keys.pressed(KeyCode::KeyS) {
            vel.linvel.y = -pos_delta;
            vel.linvel.x = 0.;
        }
        if keys.pressed(KeyCode::KeyD) {
            vel.linvel.x = pos_delta;
            vel.linvel.y = 0.;
        }
        println!("velocity {:?}", vel)
    }
}