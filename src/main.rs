mod game;
mod systems;

use std::default;

use game::{score::events::GameOver, GamePlugin};
use systems::*;

use bevy::{app::AppExit, audio::Volume, core::Zeroable, prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "hello".to_string(),
                // resolution: (800., 600.).into(),
                // resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_event::<GameOver>()
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (transition_to_menu_state, transition_to_game_state))
        .add_systems(Update, (handle_game_over).run_if(in_state(AppState::Game)))
        .add_systems(Update, (exist_game))
        .run()
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
