use bevy::{
    ecs::system::Resource,
    time::{Timer, TimerMode},
};

use super::ENEMY_SPAWN_TIME;

#[derive(Resource)]
pub struct enemySpawnTimer {
    pub timer: Timer,
}

impl Default for enemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
