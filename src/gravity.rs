use crate::{GameState, GameplaySet};
use bevy::{math::math, prelude::*};
use rand::prelude::*;

pub struct GravityPlugin;

pub const GRAVITY_CONST: f32 = 1;

#[derive(Component)]
pub struct GravitySource {
    max_range: f32
}

#[derive(Component)]
pub struct Velocity {
    velocity: Vec2
}

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.velocity
    }
}
impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.velocity
    }
}

#[derive(Component)]
pub struct Acceleration {
    acceleration: Vec2
}

impl Deref for Acceleration {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.acceleration
    }
}
impl DerefMut for Acceleration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.acceleration
    }
}

#[derive(Component)]
pub struct Force {
    force: Vec2
}

impl Deref for Force {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.force
    }
}
impl DerefMut for Force {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.force
    }
}

#[derive(Component)]
pub struct Mass {
    mass: f32
}

impl Deref for Mass {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.mass
    }
}
impl DerefMut for Mass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mass
    }
}

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    move_enemy.run_if(in_state(GameState::Playing)),
                    spawn_enemy.run_if(in_state(GameState::Playing)),
                )
                    .in_set(GameplaySet::EnemyUpdate),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(GravitySource::new());
}

fn force_update(
    mut forces_query: Query<(&mut Force, &Mass, &Transform)>,
    gravity_query: Query<(&GravitySource, &Mass, &Transform)>
) {
    // todo: make sure translations are centered!
    // maybe skip calc on self?
    for (force, mass, transform) in forces_query.iter_mut() {
        *force = Vec2::new(0.0f, 0.0f);
        for (source_gravity, source_mass, source_transform) in gravity_query.iter() {
            let distance_vector = (transform.translation - source_transform.translation);
            let distance = distance_vector.length();
            // does this need a bound to prevent negative dist?
            if distance < source_gravity.max_range {
                let force_magnitude = GRAVITY_CONST * ((mass * source_mass) / (distance * distance));
                let force_direction = distance_vector.normalize();
                *force += force_direction * force_magnitude;
            }
        }
    }
}

fn velocity_update(
    time: Res<Time>,
    mut velocity_query: Query<(&mut Velocity, &Acceleration)>,
) {
    for (mut velocity, acceleration) in velocity_query.iter_mut() {
        *velocity += acceleration * time.delta_seconds();
        // todo: add player input?
    }
}

fn position_update(
    time: Res<Time>,
    mut transform_query: Query<(&mut Transform, &Velocity)>
) {
    for (mut transform, velocity) in transform_query.iter_mut() {
        transform.translation += velocity * time.delta_seconds();
    }
}
