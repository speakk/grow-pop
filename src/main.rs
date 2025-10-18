use bevy::input_focus::InputDispatchPlugin;
#[allow(unused_imports)]
#[cfg(debug_assertions)] // new
use bevy_dylib;

use crate::splash::splash_plugin;
use crate::start_menu::start_menu_plugin;
use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;
use crate::ui_widgets::menu_button::menu_button_plugin;

mod splash;
mod start_menu;
mod ui_widgets;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiWidgetsPlugins, InputDispatchPlugin))
        .add_plugins(splash_plugin)
        .add_plugins(start_menu_plugin)
        .add_plugins(menu_button_plugin)
        .init_state::<GameState>()
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    StartMenu,
    InGame,
}
