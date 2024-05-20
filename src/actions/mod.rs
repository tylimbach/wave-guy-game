use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::actions::game_control::{get_control_pressed, GameControl};
use crate::player::Player;
use crate::GameState;
use crate::GameplaySet::InputHandling;

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .init_resource::<MouseCoords>()
            .add_systems(
                Update,
                (cursor_system, set_movement_actions)
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .in_set(InputHandling),
            );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub camera_movement: Option<Vec3>,
    pub shoot: Option<Vec2>,
}

#[derive(Default, Resource)]
pub struct MouseCoords {
    pub world_coords: Option<Vec2>,
    pub logical_coords: Option<Vec2>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse_world_coords: Res<MouseCoords>,
) {
    let mut player_movement = Vec2::new(
        get_control_pressed(GameControl::Right, &keyboard_input, &mouse_input)
            - get_control_pressed(GameControl::Left, &keyboard_input, &mouse_input),
        get_control_pressed(GameControl::Up, &keyboard_input, &mouse_input)
            - get_control_pressed(GameControl::Down, &keyboard_input, &mouse_input),
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
        get_control_pressed(GameControl::ZoomIn, &keyboard_input, &mouse_input)
            - get_control_pressed(GameControl::ZoomOut, &keyboard_input, &mouse_input),
    );

    // touch position affect camera?
    if camera_movement != Vec3::ZERO {
        actions.camera_movement = Some(camera_movement.normalize());
    } else {
        actions.camera_movement = None;
    }

    // shoot action
    if GameControl::MainAttack.pressed(&keyboard_input, &mouse_input) {
        actions.shoot = Some(mouse_world_coords.world_coords.unwrap());
    } else {
        actions.shoot = None;
    }
}

// todo: is our camera setup correctly?
pub fn cursor_system(
    mut coords: ResMut<MouseCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(cursor_position) = window.cursor_position() {
        coords.logical_coords = Some(cursor_position);
        coords.world_coords = camera.viewport_to_world_2d(camera_transform, cursor_position);
    }
}
