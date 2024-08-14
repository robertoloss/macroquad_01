//use std::process::exit;
mod map;
mod player;
use player::*;
use std::collections::HashMap;
use map::*;
use macroquad::{prelude::*, window};
use miniquad::window::order_quit;

fn create_game_map(map: &Vec<Vec<u8>>) -> HashMap<(u8,u8), Tile> {
    let mut tile_hash_map: HashMap<(u8,u8), Tile> = HashMap::new();
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
                );
                tile_hash_map.insert((y as u8,x as u8), new_tile);
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
        },
        go_right: false,
        go_left: false,
        go_last: HorizontalDirection::None
    };
    let game_map = create_game_map(&_map1);
    
    window::request_new_screen_size(
        _map1[0].len() as f32 * TILE_SIZE,
        (_map1.len() as f32 * TILE_SIZE) + TILE_SIZE
    );
    let mut counter: u64 = 0;
    let mut print_fps: String = "".to_string();
    let mut print_delta_time: String = "".to_string();

    loop {
        counter = counter + 1;
        let delta_time = get_frame_time() * 100.;
        if is_key_pressed(KeyCode::Right) {
            player.go_right = true;
            player.go_last = HorizontalDirection::Right;
        }
        if is_key_pressed(KeyCode::Left) {
            player.go_left = true;
            player.go_last = HorizontalDirection::Left;
        }
        if is_key_released(KeyCode::Right) {
            player.go_right = false;
            if player.go_left {
                player.go_last = HorizontalDirection::Left
            }
        } 
        if is_key_released(KeyCode::Left) {
            player.go_left = false;
            if player.go_right {
                player.go_last = HorizontalDirection::Right
            }
        }

        if !player.go_left && !player.go_right {
            player.go_last = HorizontalDirection::None
        }

        if is_key_pressed(KeyCode::Up) {}
        if is_key_pressed(KeyCode::Down) {}
        if is_key_pressed(KeyCode::Z) {
            if player.speed.y > -10.0 {
                player.speed.y = -5.
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            order_quit()
        }
        match player.go_last {
            HorizontalDirection::Right => player.speed.x = 2.5,
            HorizontalDirection::Left => player.speed.x = -2.5,
            HorizontalDirection::None => player.speed.x = 0.0,
        }
        if player.speed.y < 5. {
            player.speed.y += 0.2
        }

        player.check_collision_vertical(&game_map);
        player.check_collision_horizontal(&game_map);

        player.position.y += player.speed.y * delta_time;
        player.position.x += player.speed.x * delta_time;
        
        let fps = get_fps().to_string();


        clear_background(BLACK);
        draw_rectangle(player.position.x, player.position.y, 40.0, 40.0, RED);
        create_game_map(&_map1);
        if counter == 1 {
            print_fps = fps.clone();
            print_delta_time = delta_time.to_string();
        }
        if counter > 16 {
            counter = 0;
        }
        draw_text(&print_fps, 40.0, 60.0, 24.0, YELLOW);
        draw_text(&print_delta_time, 40.0, 80.0, 24.0, YELLOW);

        next_frame().await
    }
}
