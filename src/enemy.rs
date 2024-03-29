use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_enemy);
    }
}

fn spawn_enemy(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.monster1.clone(),
            transform: Transform::from_translation(Vec3::new(0.5, 0.5, 1.))
                .with_scale(Vec3::new(1., 1., 1.)),
            ..Default::default()
        })
        .insert(Enemy);
}

/*
fn move_enemy(
    time: Res<Time>,
    actions: Res<Actions>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
) {
    if actions.enemy_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.enemy_movement.unwrap().x * speed * time.delta_seconds(),
        actions.enemy_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut enemy_transform in &mut enemy_query {
        enemy_transform.translation += movement;
    }
}
*/
