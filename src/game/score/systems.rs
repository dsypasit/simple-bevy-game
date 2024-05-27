use bevy::prelude::*;

use crate::main_menu::styles::get_text_style;

use super::{
    components::{ScoreBoard, ScoreText},
    resources::{HighestScore, Score},
};

pub fn highest_score_updated(highest_score: Res<HighestScore>) {
    if highest_score.is_changed() {
        println!("highest score updated! : {}", highest_score.value)
    }
}

pub fn insert_resource(mut commands: Commands) {
    commands.insert_resource(Score::default());
    commands.insert_resource(HighestScore::default());
}

pub fn remove_resource(mut commands: Commands) {
    commands.remove_resource::<Score>();
    commands.remove_resource::<HighestScore>();
}

pub fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    show_score(&mut commands, &asset_server)
}

pub fn despawn_score(mut commands: Commands, query: Query<Entity, With<ScoreBoard>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive()
    }
}

pub fn show_score(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    margin: UiRect::new(Val::Px(10.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                    ..default()
                },
                ..default()
            },
            ScoreBoard {},
        ))
        .with_children(|p| {
            // text
            p.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Score: ", get_text_style(asset_server))],
                    ..default()
                },
                ..default()
            });

            // text
            p.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("0", get_text_style(asset_server))],
                        ..default()
                    },
                    ..default()
                },
                ScoreText {},
            ));
        });
}

pub fn update_score(score: Res<Score>, mut score_text_query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut score_text) = score_text_query.get_single_mut() {
        if score.is_changed() {
            score_text.sections[0].value = format!("{}", score.value);
        }
    }
}
