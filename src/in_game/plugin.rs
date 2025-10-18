use crate::in_game::map::{InTilePosition, Map};
use crate::GameState;
use bevy::prelude::*;
use crate::in_game::plant::Plant;

pub fn in_game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), setup)
        .add_observer(grow_example_plants);
}

fn setup(mut commands: Commands) {
    fastrand::seed(7); // TODO: Map seed
    commands.spawn((Map::default(), DespawnOnExit(GameState::InGame)));
}

fn grow_example_plants(add: On<Add, Map>, map: Query<&Map>, mut commands: Commands) {
    let mut successes = 0;
    let mut attempts = 0;
    let max_attempts = 500;
    let amount = 5;
    let map = map.get(add.entity).unwrap();
    let map_size = map.size;

    while successes < amount && attempts < max_attempts {
        attempts += 1;
        let rand_position = IVec2::new(
            fastrand::i32(-map_size.x/2..map_size.x/2),
            fastrand::i32(-map_size.y/2..map_size.y/2),
        );

        if let Some(tile) = map.map_data.get(&rand_position) {
            if tile.is_empty() {
                commands.spawn((Plant, InTilePosition(rand_position)));
                successes += 1;
            }
        }
    }
}