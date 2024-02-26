use bevy::{
    prelude::*, 
    input::keyboard::keyboard_input_system, 
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use player::{player::InputControllable, player_input::player_input};
use bevy_rapier2d::prelude::*;

pub mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
  //      #.add_plugins(LogDiagnosticsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, player_input)
        .add_systems(Startup, setup)
        .run();
}

const X_EXTENT: f32 = 600.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::new(0.0, 0.0);
    commands.spawn(Camera2dBundle::default());

    let tile_texture = asset_server.load("resources/graphics/tile_set_earth1.png");

    for i in 1i16..200i16 {
        for j in 1i16..100i16 {
            commands.spawn((
                SpriteBundle {
                    texture: tile_texture.clone(),
                    transform: Transform::from_xyz(f32::try_from(i).unwrap()*16. - 500.,f32::try_from(j).unwrap()*16.- 500.,-1.),
                    ..default()
                },

            ));
        }
    }

    let shapes = [
        (Mesh2dHandle(meshes.add(Circle { radius: 50.0 })), Collider::ball(50.0)),
        (Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))), Collider::capsule_y(25.0, 50.0)),
        (Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))), Collider::capsule_y(25.0, 25.0)),
        (Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))), Collider::cuboid(25.0, 50.0)),
        (Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))), Collider::ball(50.0)),
        (Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))), Collider::triangle(Vec2::Y * 50.0, Vec2::new(-50.0, -50.0), Vec2::new(50.0, -50.0))),
    ];
    let num_shapes = shapes.len();

    for (i, (shape, collider)) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        },
        collider)
        );
    }
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 30.0 })),
            material: materials.add(Color::rgb(1., 1., 0.)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Collider::ball(30.0),
        RigidBody::Dynamic,
        Velocity::default(),
        InputControllable)
    );
}
