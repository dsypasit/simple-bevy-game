use bevy::{app::AppExit, prelude::*};

use crate::{
    main_menu::{
        components::*,
        styles::{HOVER_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PREESED_BUTTON_COLOR},
    },
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PREESED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            Interaction::Hovered => *background_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut event_app_exist: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::RED.into();
                event_app_exist.send(AppExit);
            }
            Interaction::Hovered => *background_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
