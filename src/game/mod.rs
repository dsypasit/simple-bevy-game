pub mod components;
pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;
pub mod weapon;
use bevy::prelude::*;
use systems::*;

use crate::AppState;

use self::{enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_systems(OnEnter(AppState::Game), game_begin)
            .add_plugins((ScorePlugin, EnemyPlugin, PlayerPlugin, StarPlugin))
            .add_systems(OnEnter(SimulationState::Pause), (spawn_pause_window))
            .add_systems(OnExit(SimulationState::Pause), (despawn_pause_window))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                (interact_with_resume_button, interact_with_main_menu_button)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Pause)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Pause,
}
