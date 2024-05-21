use bevy::{
    ecs::system::Resource,
    time::{Timer, TimerMode},
};

use super::STAR_SPAWN_TIME;

#[derive(Resource)]
pub struct starSpawnTimer {
    pub timer: Timer,
}
impl Default for starSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
