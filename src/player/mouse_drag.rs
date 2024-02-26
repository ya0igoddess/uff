use bevy::{ecs::{component::Component, entity::Entity, query::With, system::{Commands, Query, Res}}, input::{mouse::MouseButton, ButtonInput}, render::camera::Camera, transform::components::{GlobalTransform, Transform}, window::{PrimaryWindow, Window}};
use bevy_rapier2d::{pipeline::QueryFilter, plugin::RapierContext};

#[derive(Component, Default)]
pub struct MouseDraggable;

#[derive(Component, Default)]
pub struct MouseDragged;

pub fn catch_drag_item(
    mut commands: Commands,
    mouse_event: Res<ButtonInput<MouseButton>>,
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    query: Query<&MouseDraggable>
) { 
    if mouse_event.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_q.single();
        let mouse_position = window.cursor_position().and_then(
            |c| { camera.viewport_to_world_2d(camera_transform, c) }
        ).unwrap_or_default();
        rapier_context.intersections_with_point(mouse_position, QueryFilter::default(), |e| {
            if query.get(e).is_ok() {
                commands.entity(e).insert(MouseDragged::default());
                false
            } else {
                true
            }
        });
    }
}

pub fn release_drag_item(
    mut commands: Commands,
    mouse_event: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&MouseDragged, Entity)>
) { 
    if mouse_event.just_released(MouseButton::Left) {
        for (_,e) in query.iter_mut() {
            commands.entity(e).remove::<MouseDragged>();
        }
    }
}

pub fn drag_item_to_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut dragged_enteties: Query<&mut Transform, With<MouseDragged>>
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_q.single();
    let mouse_position = window.cursor_position().and_then(
        |c| { camera.viewport_to_world_2d(camera_transform, c) }
    ).unwrap_or_default();
    for mut trans in dragged_enteties.iter_mut() {
        trans.translation.x = mouse_position.x;
        trans.translation.y = mouse_position.y;
    }
}