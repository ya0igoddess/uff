use bevy::{ecs::{query::{With, Without}, system::{Query, Res}}, input::{keyboard::KeyCode, ButtonInput}, render::camera::OrthographicProjection, time::Time, transform::components::Transform};
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
    }
}

pub fn camera_follow(
    mut query_camera: Query<(&OrthographicProjection, &mut Transform), Without<InputControllable>>,
    query: Query<&Transform, With<InputControllable>>
) {
    let (_, mut camera) = query_camera.single_mut();
    let pers = query.single(); 
    camera.translation.x = pers.translation.x;
    camera.translation.y = pers.translation.y;
}