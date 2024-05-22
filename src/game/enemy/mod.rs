use bevy::app::{Plugin, Update};
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit};

use crate::AppState;

use self::resources::*;
use self::systems::*;

use super::SimulationState;

pub mod components;
pub mod resources;
pub mod systems;

pub const ENEMIS_NUMBER: i32 = 5;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<enemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemy)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_movement,
                    confine_enemy_movement,
                    enemy_hit_player,
                    spawn_enemy_overtime,
                    enemy_change_direction,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), enemy_despawn);
    }
}
