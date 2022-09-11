use rand::Rng;
use std::cmp::{max, min};
use std::collections::VecDeque;

use crate::algorithms::Option;
use crate::map::{Map, TileType};

use super::MapBuilder;

#[derive(Default)]
pub struct RoomsMap {
    map: Map,
    rooms: Vec<Rect>,
    snapshots: VecDeque<Map>,
}

#[derive(Clone)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

impl MapBuilder for RoomsMap {
    fn build_map(&mut self, width: i32, height: i32, options: &[Option]) {
        self.map = Map::new(width, height);
        self.fill(options[0].value, options[1].value, options[2].value);
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

impl RoomsMap {
    fn fill(&mut self, max_rooms: i32, room_min_size: i32, room_max_size: i32) {
        self.map.tiles.fill(Some(TileType::Wall));
        self.take_snapshot();

        let mut rng = rand::thread_rng();

        for _ in 0..max_rooms {
            let w = rng.gen_range(room_min_size..room_max_size);
            let h = rng.gen_range(room_min_size..room_max_size);

            let x = rng.gen_range(1..self.map.width as i32 - w - 1) - 1;
            let y = rng.gen_range(1..self.map.height as i32 - h - 1) - 1;

            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                self.apply_room_to_map(&new_room);
                self.take_snapshot();

                /*  if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                    if rng.gen_bool(0.5) {
                        self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        self.take_snapshot();

                        self.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        self.take_snapshot();
                        self.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }*/
                self.rooms.push(new_room);
            }
        }
        // Add corridors
        for i in 0..self.rooms.len() {
            let room = &self.rooms[i];
            let (new_x, new_y) = room.center();
            let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
            if rng.gen_bool(0.5) {
                self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                self.take_snapshot();
                self.apply_vertical_tunnel(prev_y, new_y, new_x);
            } else {
                self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                self.take_snapshot();
                self.apply_horizontal_tunnel(prev_x, new_x, new_y);
            }
            self.take_snapshot();
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.map.xy_idx(x, y);
                self.map.tiles[idx] = Some(TileType::Floor);
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.map.xy_idx(x, y);
            if idx > 0 && idx < (self.map.width * self.map.height) as usize {
                self.map.tiles[idx as usize] = Some(TileType::Floor);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.map.xy_idx(x, y);
            if idx > 0 && idx < (self.map.width * self.map.height) as usize {
                self.map.tiles[idx as usize] = Some(TileType::Floor);
            }
        }
    }
}
