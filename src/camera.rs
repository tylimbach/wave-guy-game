use crate::actions::Actions;
use crate::GameState;
use crate::player::Player;
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

pub struct CameraPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                Update,
                move_camera.run_if(in_state(GameState::Playing))
            );
    }
}

fn move_camera(
    time: Res<Time>,
    actions: Res<Actions>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => Some(*transform),
        Err(e) => match e {
            QuerySingleError::NoEntities(_) => None,
            QuerySingleError::MultipleEntities(_) => {
                eprintln!("Error: Multiple player entities found!");
                None
            },
        },
    };

    for (mut camera_proj, mut camera_transform) in camera_query.iter_mut() {
        if let Some(player_transform) = player_transform {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }

        if actions.camera_movement.is_none() {
            return;
        }

        let zoom_move = actions.camera_movement.unwrap().z * time.delta_seconds();
        camera_proj.scale += zoom_move;
    };

}
