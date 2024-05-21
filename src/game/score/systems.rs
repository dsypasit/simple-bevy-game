use bevy::prelude::*;

use super::resources::{HighestScore, Score};

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        eprintln!("Score:{}", score.value)
    }
}

pub fn highest_score_updated(highest_score: Res<HighestScore>) {
    if highest_score.is_changed() {
        println!("highest score updated! : {}", highest_score.value)
    }
}
