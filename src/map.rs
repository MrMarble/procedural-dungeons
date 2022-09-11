use bevy::prelude::*;

const TILE_SIZE: i32 = 16;

#[derive(Default, Clone)]
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

    pub fn draw(&self, mut cmds: Commands, tiles: &[Entity]) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if let Some(tile) = tile {
                let (x, y) = self.idx_xy(idx);
                cmds.entity(tiles[idx]).insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
                        color: match tile {
                            TileType::Wall => Color::rgb(1., 1., 1.),
                            TileType::Floor => Color::rgb(0.15, 0.15, 0.15),
                        },
                        ..default()
                    },
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
}
