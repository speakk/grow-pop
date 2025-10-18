use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Component)]
#[require()]
pub struct Map {
    pub center_origin: Vec2,
    pub size: IVec2,
    pub map_data: HashMap<IVec2, Tile>,
}

#[derive(Component)]
pub struct InTilePosition(pub IVec2);

impl Default for Map {
    fn default() -> Self {
        let size = IVec2::new(10, 10);
        let mut map_data: HashMap<IVec2, Tile> = HashMap::new();
        for x in -size.x..=size.x {
            for y in -size.y..=size.y {
                let pos = IVec2::new(x, y);
                map_data.insert(pos, Tile::default());
            }
        }
        Map {
            center_origin: Vec2::default(),
            size: IVec2::new(10, 10),
            map_data: HashMap::new(),
        }
    }
}

impl Map {
    
}

#[derive(Default)]
pub struct Tile {
    entities: Vec<Entity>,
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}