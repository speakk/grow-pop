use bevy::prelude::*;
use crate::GameState;

#[derive(Resource)]
struct SplashScreenTimer(Timer);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), display_title)
        .add_systems(Update, switch_to_menu.run_if(in_state(GameState::Splash)));
    }

fn display_title(mut commands: Commands) {
    commands.spawn((Camera2d, DespawnOnExit(GameState::Splash)));

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
                Text::new("GROW POP"),
                TextFont {
                    font_size: 130.0,
                    ..default()
                },
            ),
            (
                Text::new("by speak"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
            )
            ],
            DespawnOnExit(GameState::Splash),
            ));

            commands.insert_resource(SplashScreenTimer(Timer::from_seconds(0.5, TimerMode::Once)));
}

fn switch_to_menu(
    mut next: ResMut<NextState<GameState>>,
    mut timer: ResMut<SplashScreenTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(GameState::StartMenu);
    }
}
