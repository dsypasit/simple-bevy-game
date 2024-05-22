pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;
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
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Pause,
}
