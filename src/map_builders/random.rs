use std::collections::VecDeque;

use crate::{algorithms::Option, map::TileType, Map};
use rand::Rng;

use super::MapBuilder;

#[derive(Default)]
pub struct RandomMap {
    map: Map,
    snapshots: VecDeque<Map>,
}

impl MapBuilder for RandomMap {
    fn build_map(&mut self, width: i32, height: i32, options: &[Option]) {
        self.map = Map::new(width, height);
        self.fill(options[0].value);
    }
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn take_snapshot(&mut self) {
        self.snapshots.push_back(self.map.clone());
    }

    fn get_snapshot_history(&self) -> VecDeque<Map> {
        self.snapshots.clone()
    }
}

impl RandomMap {
    fn fill(&mut self, ratio: i32) {
        self.map.tiles.fill(Some(TileType::Floor));
        self.take_snapshot();

        // Fill the outer edges with walls
        for x in 0..self.map.width {
            let idx_top = self.map.xy_idx(x, 0);
            let idx_bottom = self.map.xy_idx(x, self.map.height - 1);

            self.map.tiles[idx_top] = Some(TileType::Wall);
            self.map.tiles[idx_bottom] = Some(TileType::Wall);
        }

        self.take_snapshot();

        for y in 0..self.map.height {
            let idx_left = self.map.xy_idx(0, y);
            let idx_right = self.map.xy_idx(self.map.width - 1, y);

            self.map.tiles[idx_left] = Some(TileType::Wall);
            self.map.tiles[idx_right] = Some(TileType::Wall);
        }

        self.take_snapshot();

        let rand = &mut rand::thread_rng();

        // Fill the rest of the map with random walls
        for _i in 0..(self.map.width * self.map.height / ratio) {
            let x = rand.gen_range(1..self.map.width as i32 - 1);
            let y = rand.gen_range(1..self.map.height as i32 - 1);
            let idx = self.map.xy_idx(x, y);

            self.map.tiles[idx] = Some(TileType::Wall);

            if _i % 10 == 0 {
                self.take_snapshot();
            }
        }
    }
}
