use crate::actions::Actions;
use crate::gravity::{Mass, PhysicsBundle};
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.monster1.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec3::new(3., 3., 3.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(PhysicsBundle {
            mass: Mass(10.),
            ..default()
        });
}

fn shoot(
    mut commands: Commands,
    mut meshes: Res<Assets<Mesh2d>>,
    actions: Res<Actions>,
    player_query: Query<&Transform, With<Player>>,
) {
    let transform = player_query.single();

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.Add(Circle {radius: 50.0})),
            transform: transform.clone(),
            ..default()
        })
}
