use bevy::prelude::*;

use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::{get_button_style, get_image_style, get_text_style, NORMAL_BUTTON_COLOR},
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title Button
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },))
                .with_children(|p| {
                    // image
                    p.spawn(ImageBundle {
                        style: get_image_style(),
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });

                    // text
                    p.spawn(
                        (TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Escape the ball!!",
                                    get_text_style(asset_server),
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        }),
                    );

                    // image
                    p.spawn(ImageBundle {
                        style: get_image_style(),
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });
                });
            // Quit Button
            parent
                .spawn((
                    ButtonBundle {
                        style: get_button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|p| {
                    p.spawn(
                        (TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Play",
                                    get_text_style(asset_server),
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        }),
                    );
                });
            // Quit Button
            parent
                .spawn((
                    ButtonBundle {
                        style: get_button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|p| {
                    p.spawn(
                        (TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Quit",
                                    get_text_style(asset_server),
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        }),
                    );
                });
        })
        .id();
    main_menu_entity
}
