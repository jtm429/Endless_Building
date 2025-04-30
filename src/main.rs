use macroquad::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TILE_SIZE: f32 = 32.0;
const SCALE: f32 = 2.0;
const DRAWN_TILE_SIZE: f32 = TILE_SIZE * SCALE;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tilemap Example".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

// Reads a CSV file into a 2D Vec
fn load_tilemap(path: &str) -> Vec<Vec<usize>> {
    let file = File::open(path).expect("Failed to open tilemap file.");
    let reader = BufReader::new(file);

    let mut map = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line.");
        let row = line
            .split(',')
            .map(|num| num.trim().parse::<usize>().expect("Invalid number"))
            .collect();
        map.push(row);
    }

    map
}

#[macroquad::main(window_conf)]
async fn main() {
    let tilemap = load_tilemap("assets/testmap.csv");

    let map_width = tilemap[0].len(); // number of columns
    let map_height = tilemap.len();   // number of rows

    let spritesheet: Texture2D = load_texture("assets/tilemap-sprites.png").await.unwrap();
    spritesheet.set_filter(FilterMode::Nearest);
    let tiles_per_row = (spritesheet.width() / TILE_SIZE) as usize;

    loop {
        clear_background(BLACK);

        let map_pixel_width = map_width as f32 * DRAWN_TILE_SIZE;
        let map_pixel_height = map_height as f32 * DRAWN_TILE_SIZE;

        let offset_x = (screen_width() - map_pixel_width) / 2.0;
        let offset_y = (screen_height() - map_pixel_height) / 2.0;

        for (y, row) in tilemap.iter().enumerate() {
            for (x, &tile_id) in row.iter().enumerate() {
                if tile_id >= tiles_per_row * tiles_per_row {
                    continue;
                }

                let sx = (tile_id % tiles_per_row) as f32 * TILE_SIZE;
                let sy = (tile_id / tiles_per_row) as f32 * TILE_SIZE;

                draw_texture_ex(
                    &spritesheet,
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

        next_frame().await;
    }
}