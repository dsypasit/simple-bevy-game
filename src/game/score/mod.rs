use bevy::app::{Plugin, Update};

use self::resources::{HighestScore, Score};
use self::systems::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Score>()
            .init_resource::<HighestScore>()
            .add_systems(Update, (update_score, highest_score_updated));
    }
}
