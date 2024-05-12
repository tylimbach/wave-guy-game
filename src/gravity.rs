use crate::{loading::TextureAssets, GameState, GameplaySet};
use bevy::prelude::*;
use crate::movement::{Force, Mass};

pub struct GravityPlugin;

pub const GRAVITY_CONST: f32 = 1.0;

#[derive(Component)]
pub struct GravitySource {
    max_range: f32,
}

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update, 
                force_update
                    .run_if(in_state(GameState::Playing))
                    .in_set(GameplaySet::PrePhysics),
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
        .insert(GravitySource { max_range: 2500.0 })
        .insert(Mass(6000000.0));
    commands
        .spawn(SpriteBundle {
            texture: textures.black_hole.clone(),
            transform: Transform::from_translation(Vec3::new(1500., 1500., 2.))
                .with_scale(Vec3::new(5.0, 5.0, 5.0)),
            ..Default::default()
        })
        .insert(GravitySource { max_range: 15000.0 })
        .insert(Mass(250000000.0));
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

