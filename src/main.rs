mod enemy;
mod player;
mod score;
mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::events::GameOver;
use score::ScorePlugin;
use star::StarPlugin;
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
        .add_plugins((ScorePlugin, EnemyPlugin, PlayerPlugin, StarPlugin))
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exist_game, handle_game_over))
        .run()
}
