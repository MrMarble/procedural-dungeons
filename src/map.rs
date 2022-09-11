use bevy::prelude::*;

use crate::TextureMap;

const TILE_SIZE: i32 = 16;

#[derive(Default, Clone, PartialEq, Eq)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles: Vec<Option<TileType>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            tiles: vec![None; (width * height) as usize],
        }
    }

    pub fn draw(&self, mut cmds: Commands, texture: Res<TextureMap>, tiles: &[Entity]) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if let Some(tile) = tile {
                let (x, y) = self.idx_xy(idx);

                let mut sprite = match *tile {
                    TileType::Floor => TextureAtlasSprite::new(255),
                    TileType::Wall => TextureAtlasSprite::new(self.wall_texture(x, y)),
                };

                sprite.custom_size = Some(Vec2::splat(TILE_SIZE as f32));

                cmds.entity(tiles[idx]).insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture.0.clone(),
                    sprite: sprite,
                    transform: Transform::from_xyz(
                        (x * TILE_SIZE - (TILE_SIZE * self.width / 2)) as f32,
                        (y * TILE_SIZE - (TILE_SIZE * self.height / 2)) as f32,
                        0.,
                    ),
                    ..default()
                });
            }
        }
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn wall_texture(&self, x: i32, y: i32) -> usize {
        if x < 1 || x > self.width - 2 || y < 1 || y > self.height - 2 as i32 {
            return 0;
        }
        let mut mask: u8 = 0;
        if self.is_wall(x, y + 1) {
            mask += 1;
        }
        if self.is_wall(x, y - 1) {
            mask += 2;
        }
        if self.is_wall(x - 1, y) {
            mask += 4;
        }
        if self.is_wall(x + 1, y) {
            mask += 8;
        }

        match mask {
            0 => 9,    // Pillar because we can't see neighbors
            1 => 208,  // Wall only to the north
            2 => 210,  // Wall only to the south
            3 => 186,  // Wall to the north and south
            4 => 181,  // Wall only to the west
            5 => 188,  // Wall to the north and west
            6 => 187,  // Wall to the south and west
            7 => 185,  // Wall to the north, south and west
            8 => 198,  // Wall only to the east
            9 => 200,  // Wall to the north and east
            10 => 201, // Wall to the south and east
            11 => 204, // Wall to the north, south and east
            12 => 205, // Wall to the east and west
            13 => 202, // Wall to the east, west, and south
            14 => 203, // Wall to the east, west, and north
            15 => 206, // ╬ Wall on all sides
            _ => 0,    // We missed one?
        }
    }
    fn is_wall(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        if let Some(tile) = self.tiles[idx].as_ref() {
            *tile == TileType::Wall
        } else {
            false
        }
    }
}
