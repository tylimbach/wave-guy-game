use crate::{actions::Actions, loading::TextureAssets, player::Player, GameState, GameplaySet};
use bevy::prelude::*;

pub struct GravityPlugin;

pub const GRAVITY_CONST: f32 = 1.0;

#[derive(Component)]
pub struct GravitySource {
    max_range: f32,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct Force(pub Vec3);

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

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    force_update,
                    force_update_input.after(force_update),
                    acceleration_update.after(force_update_input),
                    velocity_update.after(acceleration_update),
                    position_update.after(velocity_update),
                )
                    .run_if(in_state(GameState::Playing))
                    .in_set(GameplaySet::Physics),
            );
    }
}

fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.black_hole.clone(),
            transform: Transform::from_translation(Vec3::new(-500., 0., 2.))
                .with_scale(Vec3::new(1., 1., 1.)),
            ..Default::default()
        })
        .insert(GravitySource { max_range: 2000.0 })
        .insert(Mass(2000000.0));
    commands
        .spawn(SpriteBundle {
            texture: textures.black_hole.clone(),
            transform: Transform::from_translation(Vec3::new(-300., -400., 2.))
                .with_scale(Vec3::new(1.5, 1.5, 1.5)),
            ..Default::default()
        })
        .insert(GravitySource { max_range: 2000.0 })
        .insert(Mass(4000000.0));
}

fn force_update(
    mut forces_query: Query<(&mut Force, &Mass, &Transform)>,
    gravity_query: Query<(&GravitySource, &Mass, &Transform)>,
) {
    for (mut force, mass, transform) in forces_query.iter_mut() {
        force.0 = Vec3::new(0.0, 0.0, 0.0);
        for (source_gravity, source_mass, source_transform) in gravity_query.iter() {
            let mut force_direction = source_transform.translation - transform.translation;
            let distance = force_direction.length();

            if transform.translation == source_transform.translation {
                continue;
            }
            // does this need a bound to prevent negative dist?
            if distance < source_gravity.max_range {
                let force_magnitude =
                    GRAVITY_CONST * ((mass.0 * source_mass.0) / (distance * distance));
                force_direction = force_direction.normalize();
                force.0 += force_direction * force_magnitude;
            }
        }
    }
}

fn force_update_input(actions: Res<Actions>, mut player_query: Query<&mut Force, With<Player>>) {
    if actions.player_movement.is_none() {
        return;
    }

    let force_magnitude = 100.0;
    let movement_force = Vec3::new(
        actions.player_movement.unwrap().x * force_magnitude,
        actions.player_movement.unwrap().y * force_magnitude,
        0.,
    );
    for mut force in player_query.iter_mut() {
        force.0 += movement_force;
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
        // todo: add player input?
    }
}

fn position_update(time: Res<Time>, mut transform_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in transform_query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
