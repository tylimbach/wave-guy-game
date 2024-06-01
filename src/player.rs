#![allow(unused)] // todo: remove eventually

use std::time::Duration;
use crate::actions::Actions;
use crate::collision::{Collider, CollisionEvent, CollisionLayer};
use crate::loading::TextureAssets;
use crate::movement::{Mass, PhysicsBundle, Velocity};
use crate::{GameState, GameplaySet, ZLayer};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::info;
use bevy::utils::petgraph::visit::Walker;

pub const BULLET_RADIUS: f32 = 10.0;
pub const BULLET_DURATION_MS: u64 = 2000;

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
                (despawn_timed_objects, shoot, handle_bullet_collision)
                    .chain()
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
            timer: Timer::new(Duration::from_millis(50), TimerMode::Once),
            speed: 1000.0,
            damage: 50.0,
        });
}

#[derive(Component)]
pub struct Bullet { 
    owner: Entity,
    damage: f32,
}

#[derive(Component)]
pub struct Weapon {
    timer: Timer,
    speed: f32,
    damage: f32,
}

#[derive(Component)]
pub struct SpawnLocation {
    xy: Vec2
}

#[derive(Component)]
pub struct LifeSpan {
    timer: Timer
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    actions: Res<Actions>,
    mut player_query: Query<(&GlobalTransform, &mut Weapon, Entity), With<Player>>,
    time: Res<Time>, 
) {
    let (player_transform, mut weapon, player_entity) = player_query.single_mut();
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
                .insert(Bullet { 
                    owner: player_entity,
                    damage: weapon.damage 
                })
                .insert(PhysicsBundle {
                    mass: Mass(10.),
                    velocity: Velocity(velocity_vec),
                    ..default()
                })
                .insert(Collider::new_circle(
                    CollisionLayer::PlayerProjectile,
                    BULLET_RADIUS,
                ))
                .insert(LifeSpan {
                    timer: Timer::new(Duration::from_millis(BULLET_DURATION_MS), TimerMode::Once)
                });
            
            weapon.timer.reset();
        }
    }
}

fn despawn_timed_objects(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LifeSpan)>,
    time: Res<Time>,
) {
    for (entity, mut lifespan) in query.iter_mut() {
        lifespan.timer.tick(time.delta());
        if lifespan.timer.finished() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();        
            } else {
                error!("Error getting entity to despawn");
            }
        }
    }
}

fn handle_bullet_collision(
    mut bullet_query: Query<(&mut Velocity, &mut Transform, &Bullet)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for e in collision_events.read() {
        assert_ne!(e.entity1, e.entity2);
        unsafe {
            let Ok((mut velocity1, mut transform1, bullet1)) = bullet_query.get_unchecked(e.entity1) else {
                continue;
            };
            let Ok((mut velocity2, mut transform2, bullet2)) = bullet_query.get_unchecked(e.entity2) else {
                continue;
            };

            let normal = e.normal;
            let velocity1_reflected = e.velocity1 - 2.0 * e.velocity1.dot(normal) * normal;
            let velocity2_reflected = e.velocity2 - 2.0 * e.velocity2.dot(normal) * normal;

            velocity1.0 = velocity1_reflected;
            velocity2.0 = velocity2_reflected;

            // todo: move bullets apart to avoid overlap?
            transform1.translation -= (normal * BULLET_RADIUS / 2.0).extend(0.0);
            transform2.translation += (normal * BULLET_RADIUS / 2.0).extend(0.0);
        }
    }
}
