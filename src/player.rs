#![allow(unused)] // todo: remove eventually

use std::time::Duration;
use crate::actions::Actions;
use crate::collision::{Collider, CollisionLayer};
use crate::loading::TextureAssets;
use crate::movement::{Mass, PhysicsBundle, Velocity};
use crate::{GameState, GameplaySet, ZLayer};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::info;

pub const BULLET_RADIUS: f32 = 10.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                shoot
                    .in_set(GameplaySet::PlayerUpdate)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    image_assets: Res<Assets<Image>>,
) {
    let texture = textures.player.clone();
    let Some(image_data) = image_assets.get(texture.clone()) else {
        panic!("Failed to get image data for player spawn");
    };
    let size = Vec2::new(
        image_data.texture_descriptor.size.width as f32,
        image_data.texture_descriptor.size.height as f32,
    );

    let sprite = SpriteBundle {
        texture,
        transform: Transform::from_translation(Vec3::new(0., 0., ZLayer::Character.into()))
            .with_scale(Vec3::new(3., 3., 3.)),
        ..Default::default()
    };

    commands
        .spawn(sprite)
        .insert(Player)
        .insert(PhysicsBundle {
            mass: Mass(10.),
            ..default()
        })
        .insert(Collider::new_aabb(CollisionLayer::Player, size / 2.0))
        .insert(Weapon {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
            speed: 300.0,
            damage: 50.0,
        });
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Bullet {
    damage: f32,
}

#[derive(Component)]
pub struct Weapon {
    timer: Timer,
    speed: f32,
    damage: f32,
}

/*
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct SpawnPosition(Vec3);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct LifeTime(Timer);
*/

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    actions: Res<Actions>,
    mut player_query: Query<(&GlobalTransform, &mut Weapon), With<Player>>,
    time: Res<Time>, 
) {
    let (player_transform, mut weapon) = player_query.single_mut();
    weapon.timer.tick(time.delta());
    
    if let Some(shoot_coord) = actions.shoot {
        if weapon.timer.finished() {
            let direction_vec = (shoot_coord - player_transform.translation().truncate()).normalize();
            let velocity_vec = direction_vec * weapon.speed;
            let color = Color::hsl(0.5, 0.95, 0.7);
            let handle = Mesh2dHandle(meshes.add(Circle::new(BULLET_RADIUS)));

            // todo: precreate these resources
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: handle,
                    transform: Transform::from_translation(player_transform.translation()),
                    material: materials.add(color),
                    ..default()
                })
                .insert(Bullet { damage: weapon.damage })
                .insert(PhysicsBundle {
                    mass: Mass(10.),
                    velocity: Velocity(velocity_vec),
                    ..default()
                })
                .insert(Collider::new_circle(
                    CollisionLayer::PlayerProjectile,
                    BULLET_RADIUS,
                ));
            
            weapon.timer.reset();
        }
    }
}
