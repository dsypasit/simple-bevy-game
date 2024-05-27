use bevy::app::{Plugin, Update};
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit};

use crate::AppState;

use self::resources::{HighestScore, Score};
use self::systems::*;

use super::SimulationState;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Game), (insert_resource, spawn_score))
            .add_systems(
                Update,
                (update_score, highest_score_updated).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), (remove_resource, despawn_score));
    }
}
