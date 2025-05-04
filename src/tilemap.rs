use macroquad::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TILE_SIZE: f32 = 32.0;
const SCALE: f32 = 2.0;
const DRAWN_TILE_SIZE: f32 = TILE_SIZE * SCALE;

pub struct TileMap {
    pub tiles: Vec<Vec<usize>>,
    pub spritesheet: Texture2D,
    pub tiles_per_row: usize,
}

impl TileMap {
    pub async fn load(csv_path: &str, sprite_path: &str) -> Self {
        // Load tilemap from CSV
        let file = File::open(csv_path).expect("Failed to open tilemap file.");
        let reader = BufReader::new(file);

        let mut tiles = Vec::new();
        for line in reader.lines() {
            let line = line.expect("Failed to read line.");
            let row = line
                .split(',')
                .map(|num| num.trim().parse::<usize>().expect("Invalid number"))
                .collect();
            tiles.push(row);
        }

        // Load sprite sheet
        let spritesheet = load_texture(sprite_path).await.unwrap();
        spritesheet.set_filter(FilterMode::Nearest);
        let tiles_per_row = (spritesheet.width() / TILE_SIZE) as usize;

        TileMap {
            tiles,
            spritesheet,
            tiles_per_row,
        }
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile_id) in row.iter().enumerate() {
                if tile_id >= self.tiles_per_row * self.tiles_per_row {
                    continue;
                }

                let sx = (tile_id % self.tiles_per_row) as f32 * TILE_SIZE;
                let sy = (tile_id / self.tiles_per_row) as f32 * TILE_SIZE;

                draw_texture_ex(
                    &self.spritesheet,
                    offset_x + x as f32 * DRAWN_TILE_SIZE,
                    offset_y + y as f32 * DRAWN_TILE_SIZE,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(DRAWN_TILE_SIZE, DRAWN_TILE_SIZE)),
                        source: Some(Rect::new(sx, sy, TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn width(&self) -> usize {
        self.tiles.first().map(|row| row.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn drawn_tile_size() -> f32 {
        DRAWN_TILE_SIZE
    }
}