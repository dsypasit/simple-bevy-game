pub mod components;
pub mod styles;
pub mod systems;

use bevy::prelude::*;

use crate::AppState;

use self::systems::{
    interaction::{interact_with_play_button, interact_with_quit_button},
    layout::{despawn_main_menu, spawn_main_menu},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(
            Update,
            (interact_with_play_button, interact_with_quit_button)
                .run_if(in_state(AppState::MainMenu)),
        );
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
