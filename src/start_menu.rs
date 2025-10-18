use bevy::prelude::*;
use bevy::ui_widgets::{observe, Activate};
use crate::GameState;
use crate::ui_widgets::menu_button::menu_button;

pub fn start_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::StartMenu), display_menu);
    }

fn display_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, DespawnOnExit(GameState::StartMenu)));

    commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
            (
                menu_button(&*asset_server, "New Game".to_string()),
                observe(|_activate: On<Activate>, mut next: ResMut<NextState<GameState>>| {
                    info!("Activate?");
                    next.set(GameState::InGame);
                })
            ),
            (
                menu_button(&*asset_server, "Quit".to_string()),
                observe(|_activate: On<Activate>, mut exit: MessageWriter<AppExit>| {
                    exit.write(AppExit::Success);
                })
            )
            ],
            DespawnOnExit(GameState::StartMenu),
            ));
}