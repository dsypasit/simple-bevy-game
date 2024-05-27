use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const HOVER_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PREESED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn get_button_style() -> Style {
    Style {
        height: Val::Px(80.0),
        width: Val::Px(200.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Style::default()
    }
}

pub fn get_image_style() -> Style {
    Style {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
        margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
        ..default()
    }
}

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
