use crate::gravity::{Mass};
use crate::loading::TextureAssets;
use crate::player::Player;
use crate::{GameState, GameplaySet};
use bevy::prelude::*;
use rand::prelude::*;
use crate::movement::PhysicsBundle;

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
) {
    // could cache this
    let mut rng = rand::thread_rng();
    for mut spawner in spawner_query.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.finished() {
            let rand_x = rng.gen_range(-1000.0..1000.0);
            let rand_y = rng.gen_range(-1000.0..1000.0);
            commands
                .spawn(SpriteBundle {
                    texture: textures.monster1.clone(),
                    transform: Transform::from_translation(Vec3::new(rand_x, rand_y, 1.))
                        .with_scale(Vec3::new(1., 1., 1.)),
                    ..Default::default()
                })
                .insert(Enemy)
                .insert(PhysicsBundle {
                    mass: Mass(5.),
                    ..default()
                });
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
