use bracket_pathfinding::prelude::DijkstraMap;

use crate::map::{Map, TileType};

use super::Rect;

pub fn apply_room_to_map(map: &mut Map, room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = Some(TileType::Floor);
        }
    }
}

/// Searches a map, removes unreachable areas and returns the most distant tile.
pub fn remove_unreachable_areas_returning_most_distant(map: &mut Map, start_idx: usize) {
    let map_starts: Vec<usize> = vec![start_idx];
    let dijkstra_map = DijkstraMap::new(
        map.width as usize,
        map.height as usize,
        &map_starts,
        map,
        200.0,
    );
    let mut exit_tile = (0, 0.0f32);
    for (i, tile) in map.tiles.iter_mut().enumerate() {
        if *tile == Some(TileType::Floor) {
            let distance_to_start = dijkstra_map.map[i];
            // We can't get to this tile - so we'll make it a wall
            if distance_to_start == std::f32::MAX {
                *tile = Some(TileType::Wall);
            } else {
                // If it is further away than our current exit candidate, move the exit
                if distance_to_start > exit_tile.1 {
                    exit_tile.0 = i;
                    exit_tile.1 = distance_to_start;
                }
            }
        }
    }
}
