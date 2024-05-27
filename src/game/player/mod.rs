use bevy::app::Plugin;

use crate::*;

use self::{components::CreateBulletEvent, systems::*};

use super::{enemy::systems::confine_enemy_movement, SimulationState};

pub mod components;
pub mod resources;
pub mod systems;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const BULLET_SPEED: f32 = 1000.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CreateBulletEvent>()
            .add_systems(OnEnter(AppState::Game), (spawn_player))
            .add_systems(
                Update,
                ((
                    player_movement.before(confine_player_movement),
                    confine_player_movement,
                    shoot,
                    bullet_direction,
                    bullet_hit_screen,
                    bullet_hit_enemy,
                    create_bullet,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::Game), (player_despawn, bullet_despawn));
    }
}
