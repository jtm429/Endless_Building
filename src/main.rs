mod tilemap;

use macroquad::prelude::*;
use tilemap::TileMap;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tilemap Example".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tilemap = TileMap::load("assets/testmap.csv", "assets/tilemap-sprites.png").await;

    let map_pixel_width = tilemap.width() as f32 * TileMap::drawn_tile_size();
    let map_pixel_height = tilemap.height() as f32 * TileMap::drawn_tile_size();

    loop {
        clear_background(BLACK);

        let offset_x = (screen_width() - map_pixel_width) / 2.0;
        let offset_y = (screen_height() - map_pixel_height) / 2.0;

        tilemap.draw(offset_x, offset_y);

        next_frame().await;
    }
}