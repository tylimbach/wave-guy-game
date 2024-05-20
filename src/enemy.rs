use crate::collision::{Collider, CollisionLayer, HitBox};
use crate::loading::TextureAssets;
use crate::map::MAP_RADIUS;
use crate::movement::{Mass, PhysicsBundle};
use crate::player::Player;
use crate::{GameState, GameplaySet, ZLayer};
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use rand::prelude::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Spawner {
    timer: Timer,
}

impl Plugin for EnemyPlugin {
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

impl Spawner {
    fn new(frequency_s: f32) -> Self {
        Spawner {
            timer: Timer::from_seconds(frequency_s, TimerMode::Repeating),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Spawner::new(5.0));
}

fn spawn_enemy(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    time: Res<Time>,
    mut spawner_query: Query<&mut Spawner>,
    image_assets: Res<Assets<Image>>,
) {
    // could cache this
    let mut rng = rand::thread_rng();
    for mut spawner in spawner_query.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.finished() {
            let rand_x = rng.gen_range(-MAP_RADIUS..MAP_RADIUS);
            let rand_y = rng.gen_range(-MAP_RADIUS..MAP_RADIUS);
            let texture = textures.monster1.clone();
            let Some(image_data) = image_assets.get(texture.clone()) else {
                panic!("Failed to get image data for enemy spawn");
            };
            let size = Vec2::new(
                image_data.texture_descriptor.size.width as f32,
                image_data.texture_descriptor.size.height as f32,
            );

            commands
                .spawn(SpriteBundle {
                    texture,
                    transform: Transform::from_translation(Vec3::new(
                        rand_x,
                        rand_y,
                        f32::from(ZLayer::Character) + 1.0,
                    )),
                    ..Default::default()
                })
                .insert(Enemy)
                .insert(PhysicsBundle {
                    mass: Mass(5.),
                    ..default()
                })
                .insert(Collider::new_aabb(CollisionLayer::Enemy, size / 2.0));
        }
    }
}

fn move_enemy(
    time: Res<Time>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    // todo: change to use force
    let speed = 100.;

    let player_translation = player_query.single().translation;

    for mut enemy_transform in enemy_query.iter_mut() {
        let direction = (player_translation - enemy_transform.translation).normalize_or_zero();
        let movement = direction * speed * time.delta_seconds();
        enemy_transform.translation += movement;
    }
}
