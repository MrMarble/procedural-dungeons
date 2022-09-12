use std::collections::VecDeque;

use rand::{rngs::ThreadRng, Rng};

use crate::map::{Map, TileType};

use super::{MapBuilder, Rect};

#[derive(Default)]
pub struct BspInteriorMap {
    map: Map,
    rooms: Vec<Rect>,
    rects: Vec<Rect>,
    snapshots: VecDeque<Map>,
}

impl MapBuilder for BspInteriorMap {
    fn build_map(&mut self, width: i32, height: i32, options: &[crate::algorithms::Option]) {
        self.map = Map::new(width, height);
        self.build(options[0].value, options[1].value as f64 / 10.);
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

impl BspInteriorMap {
    fn build(&mut self, min_room_size: i32, split_chance: f64) {
        self.map.tiles.fill(Some(TileType::Wall));
        self.take_snapshot();
        let rng = &mut rand::thread_rng();

        self.rects.clear();
        self.rects
            .push(Rect::new(1, 1, self.map.width - 2, self.map.height - 2));

        let first_room = self.rects[0];
        self.add_subrects(first_room, rng, min_room_size, split_chance);

        let rooms = self.rects.clone();
        for r in rooms.iter() {
            let room = *r;
            self.rooms.push(room);
            for y in room.y1..room.y2 {
                for x in room.x1..room.x2 {
                    let idx = self.map.xy_idx(x, y);
                    if idx > 0 && idx < ((self.map.width * self.map.height) - 1) as usize {
                        self.map.tiles[idx] = Some(TileType::Floor);
                    }
                }
            }
            self.take_snapshot();
        }

        if self.rooms.is_empty() {
            // No rooms were created, so we can't make corridors
            return;
        }

        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = room.x1 + (rng.gen_range(1..i32::abs(room.x1 - room.x2)) - 1);
            let start_y = room.y1 + (rng.gen_range(1..i32::abs(room.y1 - room.y2)) - 1);
            let end_x =
                next_room.x1 + (rng.gen_range(1..i32::abs(next_room.x1 - next_room.x2)) - 1);
            let end_y =
                next_room.y1 + (rng.gen_range(1..i32::abs(next_room.y1 - next_room.y2)) - 1);
            self.draw_corridor(start_x, start_y, end_x, end_y);
            self.take_snapshot();
        }
    }

    fn add_subrects(
        &mut self,
        rect: Rect,
        rng: &mut ThreadRng,
        min_room_size: i32,
        split_chance: f64,
    ) {
        // Remove the last rect from the list
        if !self.rects.is_empty() {
            self.rects.remove(self.rects.len() - 1);
        }

        // Calculate boundaries
        let width = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;
        let half_width = width / 2;
        let half_height = height / 2;

        let split = rng.gen_bool(split_chance);

        if split {
            // Horizontal split
            let h1 = Rect::new(rect.x1, rect.y1, half_width - 1, height);
            self.rects.push(h1);
            if half_width > min_room_size {
                self.add_subrects(h1, rng, min_room_size, split_chance);
            }
            let h2 = Rect::new(rect.x1 + half_width, rect.y1, half_width, height);
            self.rects.push(h2);
            if half_width > min_room_size {
                self.add_subrects(h2, rng, min_room_size, split_chance);
            }
        } else {
            // Vertical split
            let v1 = Rect::new(rect.x1, rect.y1, width, half_height - 1);
            self.rects.push(v1);
            if half_height > min_room_size {
                self.add_subrects(v1, rng, min_room_size, split_chance);
            }
            let v2 = Rect::new(rect.x1, rect.y1 + half_height, width, half_height);
            self.rects.push(v2);
            if half_height > min_room_size {
                self.add_subrects(v2, rng, min_room_size, split_chance);
            }
        }
    }

    fn draw_corridor(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            if x < x2 {
                x += 1;
            } else if x > x2 {
                x -= 1;
            } else if y < y2 {
                y += 1;
            } else if y > y2 {
                y -= 1;
            }

            let idx = self.map.xy_idx(x, y);
            self.map.tiles[idx] = Some(TileType::Floor);
        }
    }
}
