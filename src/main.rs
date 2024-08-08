//use std::process::exit;
mod map;
mod player;
use player::*;
use std::collections::HashMap;
use map::*;
use macroquad::{prelude::*, window};
use miniquad::window::order_quit;

pub const TILE_SIZE: f32 = 40.0;
pub struct U8Vec2 {
    pub x: u8,
    pub y: u8
}
pub struct Tile {
    size: f32,
    abs_pos: Vec2,
    tile_pos: U8Vec2
}
impl Default for Tile {
    fn default() -> Self {
        Tile {
            size: TILE_SIZE,
            abs_pos: Vec2 { x: 0.0, y: 0.0 },
            tile_pos: U8Vec2 { x: 0, y: 0 }
        }
    }
}
fn create_game_map(map: &Vec<Vec<u8>>) -> HashMap<(u8,u8),Tile> {
    let tile_hash_map: HashMap<(u8,u8), Tile> = HashMap::new();
    for (y, row) in map.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            if *tile == 1 {
                let mut new_tile = Tile::default();
                new_tile.tile_pos = U8Vec2 { x: x as u8, y: y as u8 };
                new_tile.abs_pos = Vec2 {
                    x: (x as f32) * TILE_SIZE,
                    y: (y as f32) * TILE_SIZE
                };
                draw_rectangle(
                    new_tile.abs_pos.x, 
                    new_tile.abs_pos.y, 
                    new_tile.size, 
                    new_tile.size, 
                    GRAY, 
                )
            }
        }
    }
    tile_hash_map
}


#[macroquad::main("macro1")]
async fn main() {

    let _map1 = get_map();
    let mut player =  Player {
        position: Vec2 {
            x: 100.,
            y: 0.
        },
        speed: Vec2 {
            x: 0.,
            y: 0.
        }
    };
    let game_map = create_game_map(&_map1);
    
    window::request_new_screen_size(
        _map1[0].len() as f32 * TILE_SIZE,
        (_map1.len() as f32 * TILE_SIZE) + TILE_SIZE
    );

    loop {
        if is_key_pressed(KeyCode::Right) {
            player.speed.x = 3.
        }
        if is_key_pressed(KeyCode::Left) {
            player.speed.x += -3.
        }
        if is_key_released(KeyCode::Right) || is_key_released(KeyCode::Left) {
            player.speed.x = 0.
        }
        if is_key_pressed(KeyCode::Up) {}
        if is_key_pressed(KeyCode::Down) {}
        if is_key_pressed(KeyCode::Z) {
            player.speed.y -= 8.
        }
        if is_key_pressed(KeyCode::Escape) {
            order_quit()
        }

        if player.speed.y < 3. {
            player.speed.y += 0.5
        }
        player.position.y += player.speed.y;
        player.position.x += player.speed.x;


        clear_background(BLACK);
        draw_rectangle(player.position.x, player.position.y, 40.0, 40.0, RED);
        create_game_map(&_map1);

        next_frame().await
    }
}
