use bevy::app::{Plugin, Startup, Update};

use self::resources::*;
use self::systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub const ENEMIS_NUMBER: i32 = 4;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<enemySpawnTimer>()
            .add_systems(Startup, spawn_enemy)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_movement,
                    confine_enemy_movement,
                    enemy_hit_player,
                    spawn_enemy_overtime,
                    enemy_change_direction,
                ),
            );
    }
}
