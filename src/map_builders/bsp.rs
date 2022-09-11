use std::collections::VecDeque;

use rand::{rngs::ThreadRng, Rng};

use crate::{
    map::{Map, TileType},
    map_builders::utils::apply_room_to_map,
};

use super::{MapBuilder, Rect};

#[derive(Default)]
pub struct BspMap {
    map: Map,
    rooms: Vec<Rect>,
    rects: Vec<Rect>,
    snapshots: VecDeque<Map>,
}

impl MapBuilder for BspMap {
    fn build_map(&mut self, width: i32, height: i32, options: &[crate::algorithms::Option]) {
        self.map = Map::new(width, height);
        self.build(options[0].value);
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

impl BspMap {
    fn build(&mut self, rooms: i32) {
        self.map.tiles.fill(Some(TileType::Wall));
        self.take_snapshot();
        let rng = &mut rand::thread_rng();

        const PADDING: i32 = 5;
        self.rects.clear();
        self.rects.push(Rect::new(
            0,
            0,
            self.map.width - PADDING,
            self.map.height - PADDING,
        ));

        let first_room = self.rects[0];
        self.add_subrects(first_room);

        let mut n_rooms = 0;
        while n_rooms < rooms {
            let rect = self.get_random_rect(rng);
            let candidate = self.get_random_sub_rect(rect, rng);

            if self.is_possible(candidate) {
                apply_room_to_map(&mut self.map, &candidate);
                self.rooms.push(candidate);
                self.add_subrects(rect);
                self.take_snapshot();
            }

            n_rooms += 1;
        }

        if self.rooms.is_empty() {
            // No rooms were created, so we can't make corridors
            return;
        }

        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));
        // Now we want corridors
        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = rng.gen_range(room.x1 + 1..room.x2);
            let start_y = rng.gen_range(room.y1 + 1..room.y2);
            let end_x = rng.gen_range(next_room.x1 + 1..next_room.x2);
            let end_y = rng.gen_range(next_room.y1 + 1..next_room.y2);
            println!("{} {} {} {}", start_x, start_y, end_x, end_y);
            self.draw_corridor(start_x, start_y, end_x, end_y);
            self.take_snapshot();
        }
    }

    fn add_subrects(&mut self, rect: Rect) {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects
            .push(Rect::new(rect.x1, rect.y1, half_width, half_height));
        self.rects.push(Rect::new(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
    }
    fn get_random_rect(&mut self, rng: &mut ThreadRng) -> Rect {
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = (rng.gen_range(1..self.rects.len() as i32) - 1) as usize;
        self.rects[idx]
    }

    fn get_random_sub_rect(&self, rect: Rect, rng: &mut ThreadRng) -> Rect {
        let mut result = rect;
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        let w = i32::max(3, rng.gen_range(1..i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(3, rng.gen_range(1..i32::min(rect_height, 10)) - 1) + 1;

        result.x1 += rng.gen_range(1..6) - 1;
        result.y1 += rng.gen_range(1..6) - 1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect: Rect) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;

        let mut can_build = true;

        for y in expanded.y1..=expanded.y2 {
            for x in expanded.x1..=expanded.x2 {
                if x > self.map.width - 2 {
                    can_build = false;
                }
                if y > self.map.height - 2 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                if can_build {
                    let idx = self.map.xy_idx(x, y);
                    if let Some(tile) = self.map.tiles[idx].as_ref() {
                        if *tile != TileType::Wall {
                            can_build = false;
                        }
                    }
                }
            }
        }

        can_build
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

            println!("{} {}", x, y);
            let idx = self.map.xy_idx(x, y);
            self.map.tiles[idx] = Some(TileType::Floor);
        }
    }
}
