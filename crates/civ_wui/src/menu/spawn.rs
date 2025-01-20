use bevy::prelude::*;

use super::Menu;

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("civ"),
                TextFont {
                    font: asset_server.load("fonts/Ubuntu-R.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.0, 0.0)),
            ));
        });
}
