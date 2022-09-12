use std::collections::VecDeque;

use rand::Rng;

use crate::map::{Map, TileType};

use super::MapBuilder;

#[derive(Default)]
pub struct CellularAutomataBuilder {
    map: Map,
    snapshots: VecDeque<Map>,
}

impl MapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self, width: i32, height: i32, options: &[crate::algorithms::Option]) {
        self.map = Map::new(width, height);
        self.build(options[0].value as f64 / 100.0);
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
    pub fn build(&mut self, floor_percent: f64) {
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
        for _i in 0..15 {
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
    }
}
