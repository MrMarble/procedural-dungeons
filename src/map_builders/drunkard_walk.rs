use std::collections::VecDeque;

use bevy::prelude::Vec2;
use bracket_pathfinding::prelude::DijkstraMap;
use rand::Rng;

use crate::map::{Map, TileType};

use super::{common::remove_unreachable_areas_returning_most_distant, MapBuilder};

pub struct DrunkardsWalkBuilder {
    map: Map,
    snapshots: VecDeque<Map>,
    starting_position: Vec2,
    settings: DrunkardSettings,
}

#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode {
    StartingPoint,
    Random,
}

pub struct DrunkardSettings {
    pub spawn_mode: DrunkSpawnMode,
    pub drunken_lifetime: i32,
    pub floor_percent: f32,
}

impl MapBuilder for DrunkardsWalkBuilder {
    fn build_map(&mut self, width: i32, height: i32, _options: &[crate::algorithms::Option]) {
        self.map = Map::new(width, height);
        self.build();
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }
    fn get_snapshot_history(&self) -> VecDeque<Map> {
        self.snapshots.clone()
    }
    fn take_snapshot(&mut self) {
        self.snapshots.push_back(self.map.clone());
    }
}

impl DrunkardsWalkBuilder {
    pub fn new(settings: DrunkardSettings) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            map: Map::default(),
            starting_position: Vec2::new(0., 0.),
            snapshots: VecDeque::new(),
            settings,
        }
    }

    pub fn open_area() -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            map: Map::default(),
            starting_position: Vec2::new(0., 0.),
            snapshots: VecDeque::new(),
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::StartingPoint,
                drunken_lifetime: 400,
                floor_percent: 0.5,
            },
        }
    }

    pub fn open_halls() -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            map: Map::default(),
            starting_position: Vec2::new(0., 0.),
            snapshots: VecDeque::new(),
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 400,
                floor_percent: 0.5,
            },
        }
    }

    pub fn winding_passages() -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            map: Map::default(),
            starting_position: Vec2::new(0., 0.),
            snapshots: VecDeque::new(),
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
            },
        }
    }

    pub fn build(&mut self) {
        let mut rng = rand::thread_rng();
        self.map.tiles.fill(Some(TileType::Wall));
        self.take_snapshot();

        self.starting_position = Vec2::new(self.map.width as f32 / 2., self.map.height as f32 / 2.);
        let start_idx = self.map.xy_idx(
            self.starting_position.x as i32,
            self.starting_position.y as i32,
        );

        let total_tiles = self.map.width * self.map.height;
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = self
            .map
            .tiles
            .iter()
            .filter(|a| **a == Some(TileType::Floor))
            .count();
        let mut digger_count = 0;
        let mut active_digger_count = 0;

        while floor_tile_count < desired_floor_tiles {
            let mut did_something = false;
            let mut drunk_x;
            let mut drunk_y;
            match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = self.starting_position.x as i32;
                    drunk_y = self.starting_position.y as i32;
                }
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = self.starting_position.x as i32;
                        drunk_y = self.starting_position.y as i32;
                    } else {
                        drunk_x = rng.gen_range(1..=self.map.width - 3) + 1;
                        drunk_y = rng.gen_range(1..=self.map.height - 3) + 1;
                    }
                }
            }
            let mut drunk_life = self.settings.drunken_lifetime;

            while drunk_life > 0 {
                let drunk_idx = self.map.xy_idx(drunk_x, drunk_y);
                if self.map.tiles[drunk_idx] == Some(TileType::Wall) {
                    did_something = true;
                }
                self.map.tiles[drunk_idx] = Some(TileType::Proggress);

                let stagger_direction = rng.gen_range(1..=4);
                match stagger_direction {
                    1 => {
                        if drunk_x > 2 {
                            drunk_x -= 1;
                        }
                    }
                    2 => {
                        if drunk_x < self.map.width - 2 {
                            drunk_x += 1;
                        }
                    }
                    3 => {
                        if drunk_y > 2 {
                            drunk_y -= 1;
                        }
                    }
                    _ => {
                        if drunk_y < self.map.height - 2 {
                            drunk_y += 1;
                        }
                    }
                }

                drunk_life -= 1;
            }
            if did_something {
                self.take_snapshot();
                active_digger_count += 1;
            }

            digger_count += 1;
            for t in self.map.tiles.iter_mut() {
                if *t == Some(TileType::Proggress) {
                    *t = Some(TileType::Floor);
                }
            }
            floor_tile_count = self
                .map
                .tiles
                .iter()
                .filter(|a| **a == Some(TileType::Floor))
                .count();
        }
        self.take_snapshot();

        remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.take_snapshot();
    }
}
