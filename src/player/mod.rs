use bevy::app::{Plugin, Startup};

use crate::*;

use self::systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 300.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, confine_player_movement));
    }
}
