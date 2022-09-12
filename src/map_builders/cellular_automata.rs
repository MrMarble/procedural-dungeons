use std::collections::VecDeque;

use bevy::prelude::Vec2;
use bracket_pathfinding::prelude::DijkstraMap;
use rand::Rng;

use crate::map::{Map, TileType};

use super::{common::remove_unreachable_areas_returning_most_distant, MapBuilder};

#[derive(Default)]
pub struct CellularAutomataBuilder {
    map: Map,
    snapshots: VecDeque<Map>,
}

impl MapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self, width: i32, height: i32, options: &[crate::algorithms::Option]) {
        self.map = Map::new(width, height);
        self.build(options[0].value as f64 / 100.0, options[1].value);
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

impl CellularAutomataBuilder {
    pub fn build(&mut self, floor_percent: f64, iterations: i32) {
        let mut rng = rand::thread_rng();
        self.map.tiles.fill(Some(TileType::Wall));
        self.take_snapshot();
        // First we completely randomize the map, setting 55% of it to be floor.
        for y in 1..self.map.height - 1 {
            for x in 1..self.map.width - 1 {
                let roll = rng.gen_bool(floor_percent);
                let idx = self.map.xy_idx(x, y);
                if roll {
                    self.map.tiles[idx] = Some(TileType::Floor)
                } else {
                    self.map.tiles[idx] = Some(TileType::Wall)
                }
            }
        }
        self.take_snapshot();

        // Now we iteratively apply cellular automata rules
        for _i in 0..iterations {
            let mut newtiles = self.map.tiles.clone();

            for y in 1..self.map.height - 1 {
                for x in 1..self.map.width - 1 {
                    let idx = self.map.xy_idx(x, y);
                    let mut neighbors = 0;
                    if self.map.tiles[idx - 1] == Some(TileType::Wall) {
                        neighbors += 1;
                    }
                    if self.map.tiles[idx + 1] == Some(TileType::Wall) {
                        neighbors += 1;
                    }
                    if self.map.tiles[idx - self.map.width as usize] == Some(TileType::Wall) {
                        neighbors += 1;
                    }
                    if self.map.tiles[idx + self.map.width as usize] == Some(TileType::Wall) {
                        neighbors += 1;
                    }
                    if self.map.tiles[idx - (self.map.width as usize - 1)] == Some(TileType::Wall) {
                        neighbors += 1;
                    }
                    if self.map.tiles[idx - (self.map.width as usize + 1)] == Some(TileType::Wall) {
                        neighbors += 1;
                    }
                    if self.map.tiles[idx + (self.map.width as usize - 1)] == Some(TileType::Wall) {
                        neighbors += 1;
                    }
                    if self.map.tiles[idx + (self.map.width as usize + 1)] == Some(TileType::Wall) {
                        neighbors += 1;
                    }

                    if neighbors > 4 || neighbors == 0 {
                        newtiles[idx] = Some(TileType::Wall);
                    } else {
                        newtiles[idx] = Some(TileType::Floor);
                    }
                }
            }

            self.map.tiles = newtiles.clone();
            self.take_snapshot();
        }
        // Find a starting point; start at the middle and walk left until we find an open tile
        let mut starting_position =
            Vec2::new(self.map.width as f32 / 2., self.map.height as f32 / 2.);
        let mut start_idx = self
            .map
            .xy_idx(starting_position.x as i32, starting_position.y as i32);
        while self.map.tiles[start_idx] != Some(TileType::Floor) {
            starting_position.x -= 1.;
            start_idx = self
                .map
                .xy_idx(starting_position.x as i32, starting_position.y as i32);
        }
        // Find all tiles we can reach from the starting point
        remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.take_snapshot();
    }
}
