use bevy::app::Plugin;

use crate::*;

use self::{resources::starSpawnTimer, systems::*};

pub mod components;
pub mod resources;
pub mod systems;

pub const STAR_SIZE: f32 = 30.0;
pub const STAR_NUMBER: i32 = 10;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<starSpawnTimer>()
            .add_systems(Startup, spawn_star)
            .add_systems(
                Update,
                (player_hit_star, spawn_star_overtime, tick_star_spawn_timer),
            );
    }
}
