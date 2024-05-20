use crate::movement::Velocity;
use crate::{GameState, GameplaySet, ZLayer};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct MapPlugin;

pub const MAP_RADIUS: f32 = 3000.0;

#[derive(Component)]
pub struct MapBoundary;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                map_boundary_system
                    .in_set(GameplaySet::Collisions)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Circle { radius: MAP_RADIUS }));
    let material = materials.add(Color::rgb(1.0, 1.0, 1.0));

    // todo: precreate these resources
    commands
        .spawn(MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, ZLayer::Map.into()),
            ..default()
        })
        .insert(MapBoundary);
}

fn map_boundary_system(
    mut query: Query<(&mut Transform, &mut Velocity), Without<MapBoundary>>,
    boundary_query: Query<&Transform, With<MapBoundary>>,
) {
    let boundary_transform = boundary_query.single();
    let boundary_position = boundary_transform.translation;
    let boundary_radius = MAP_RADIUS;

    for (mut transform, mut velocity) in query.iter_mut() {
        let direction = transform.translation - boundary_position;
        if direction.length() > boundary_radius {
            let normal = direction.normalize();
            transform.translation -= normal * ((direction.length() - boundary_radius) + 1.0); // Move entity back inside
            *velocity = Velocity(reflect_velocity(velocity.0.xy(), normal.xy()));
        }
    }
}

fn reflect_velocity(velocity: Vec2, normal: Vec2) -> Vec2 {
    velocity - 2.0 * velocity.dot(normal) * normal
}
