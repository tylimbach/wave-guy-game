use bevy::input::mouse::MouseButtonInput;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::actions::game_control::{get_one_if_pressed, GameControl};
use crate::player::Player;
use crate::GameState;

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            (
                set_movement_actions,
                set_shoot_actions
            ).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub camera_movement: Option<Vec3>,
    pub shoot: Option<Vec2>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let mut player_movement = Vec2::new(
        get_one_if_pressed(GameControl::Right, &keyboard_input, &mouse_input)
            - get_one_if_pressed(GameControl::Left, &keyboard_input, &mouse_input),
        get_one_if_pressed(GameControl::Up, &keyboard_input, &mouse_input)
            - get_one_if_pressed(GameControl::Down, &keyboard_input, &mouse_input),
    );

    if let Some(touch_position) = touch_input.first_pressed_position() {
        let (camera, camera_transform) = camera.single();
        if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
        {
            let diff = touch_position - player.single().translation.xy();
            if diff.length() > FOLLOW_EPSILON {
                player_movement = diff.normalize();
            }
        }
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }

    let camera_movement = Vec3::new(
        0.0,
        0.0,
        get_one_if_pressed(GameControl::ZoomIn, &keyboard_input, &mouse_input)
            - get_one_if_pressed(GameControl::ZoomOut, &keyboard_input, &mouse_input),
    );

    // touch position affect camera?
    if camera_movement != Vec3::ZERO {
        actions.camera_movement = Some(camera_movement.normalize());
    } else {
        actions.camera_movement = None;
    }
}

pub fn set_shoot_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let shoot_pressed = GameControl::pressed(
        &GameControl::MainAttack, 
        &keyboard_input, 
        &mouse_input
    );
    
    if shoot_pressed {
        
    }
}

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}