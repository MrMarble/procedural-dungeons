use std::collections::VecDeque;

use bevy::prelude::Vec2;
use bracket_pathfinding::prelude::DijkstraMap;
use rand::Rng;

use crate::map::{Map, TileType};

use super::{common::remove_unreachable_areas_returning_most_distant, MapBuilder};

#[derive(Default)]
pub struct DrunkardsWalkBuilder {
    map: Map,
    snapshots: VecDeque<Map>,
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
    pub fn build(&mut self) {
        let mut rng = rand::thread_rng();
        self.map.tiles.fill(Some(TileType::Wall));
        self.take_snapshot();

        let starting_position = Vec2::new(self.map.width as f32 / 2., self.map.height as f32 / 2.);
        let start_idx = self
            .map
            .xy_idx(starting_position.x as i32, starting_position.y as i32);

        let total_tiles = self.map.width * self.map.height;
        let desired_floor_tiles = (total_tiles / 2) as usize;
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
            let mut drunk_x = starting_position.x as i32;
            let mut drunk_y = starting_position.y as i32;
            let mut drunk_life = 400;

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
    }
}
