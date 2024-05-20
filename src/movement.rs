use crate::{actions::Actions, player::Player, GameState, GameplaySet};
use bevy::prelude::*;

pub struct MovementPlugin;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default)]
pub struct Force(pub Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub force: Force,
    pub mass: Mass,
    pub acceleration: Acceleration,
    pub velocity: Velocity,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            force: Force::default(),
            mass: Mass(100.),
            acceleration: Acceleration::default(),
            velocity: Velocity::default(),
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_input_velocity,
                acceleration_update.after(apply_input_velocity),
                velocity_update.after(acceleration_update),
                position_update.after(velocity_update),
            )
                .run_if(in_state(GameState::Playing))
                .in_set(GameplaySet::Physics),
        );
    }
}

fn apply_input_velocity(
    actions: Res<Actions>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let base_player_speed = 100.0;
    let movement_dir = actions.player_movement.unwrap_or(Vec2::ZERO);
    for mut velocity in player_query.iter_mut() {
        velocity.0 = movement_dir * base_player_speed;
    }
}

fn acceleration_update(mut acceleration_query: Query<(&mut Acceleration, &Force, &Mass)>) {
    for (mut acceleration, force, mass) in acceleration_query.iter_mut() {
        acceleration.0 = force.0 / mass.0;
    }
}

fn velocity_update(time: Res<Time>, mut velocity_query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in velocity_query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

fn position_update(time: Res<Time>, mut transform_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in transform_query.iter_mut() {
        transform.translation += velocity.0.extend(0.) * time.delta_seconds();
    }
}
