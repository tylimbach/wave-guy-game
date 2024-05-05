#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod camera;
mod enemy;
mod gravity;
mod loading;
mod menu;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::camera::CameraPlugin as CustomCameraPlugin;
use crate::enemy::EnemyPlugin;
use crate::gravity::GravityPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(SystemSet, Clone, Eq, PartialEq, Debug, Hash)]
enum GameplaySet {
    PlayerUpdate,
    EnemyUpdate,
    Physics,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            EnemyPlugin,
            CustomCameraPlugin,
            // GravityPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
                .configure_sets(
                    Update,
                    (
                        GameplaySet::EnemyUpdate.after(GameplaySet::PlayerUpdate),
                        GameplaySet::Physics.after(GameplaySet::EnemyUpdate),
                    ),
                );
        }
    }
}
