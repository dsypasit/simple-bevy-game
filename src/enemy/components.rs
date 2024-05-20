use bevy::time::TimerMode;
use bevy::{ecs::component::Component, math::Vec2, time::Timer};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct EnemyChangeDirection {
    pub timer: Timer,
    pub direction: f32,
}

impl Default for EnemyChangeDirection {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let time_to_change: f32 = rng.gen_range(1.0..5.0);
        let choices = [1.0, -1.0];
        let rand_direction = choices.choose(&mut rng).unwrap();
        Self {
            timer: Timer::from_seconds(time_to_change, TimerMode::Repeating),
            direction: *rand_direction,
        }
    }
}
