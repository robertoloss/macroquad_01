use std::collections::HashMap;
use::macroquad::prelude::Vec2;
use crate::map;
use map::*;

fn get_tile_coords(y: f32, x: f32) -> (u8,u8) {
   ((y/TILE_SIZE).floor() as u8, (x/TILE_SIZE).floor() as u8) 
}
pub enum HorizontalDirection {
    Left,
    Right,
    None
}
pub struct Player  {
    pub position: Vec2,
    pub speed: Vec2,
    pub go_left: bool,
    pub go_right: bool,
    pub go_last: HorizontalDirection
}
impl Player {
    pub fn check_collision_vertical(&mut self, game_map: &HashMap<(u8, u8), Tile>) {
        let top_left = game_map.get(
            &get_tile_coords(
                self.position.y + self.speed.y,
                self.position.x 
            )
        );
        let top_right = game_map.get(
            &get_tile_coords(
                self.position.y + self.speed.y,
                self.position.x + TILE_SIZE 
            )
        );
        let bottom_left = game_map.get(
            &get_tile_coords(
                self.position.y + TILE_SIZE + self.speed.y,
                self.position.x 
            )
        );
        let bottom_right = game_map.get(
            &get_tile_coords(
                self.position.y + TILE_SIZE + self.speed.y,
                self.position.x + TILE_SIZE 
            )
        );
        if let Some(tile) = top_left  {
            if self.speed.y < 0.0 {
                self.speed.y = 0.0;
                self.position.y = tile.abs_pos.y + TILE_SIZE;
                return
            }
        }
        if let Some(tile) = top_right  {
            if self.speed.y < 0.0 {
                self.speed.y = 0.0;
                self.position.y = tile.abs_pos.y + TILE_SIZE;
                return 
            } 
        }
        if let Some(tile) = bottom_left  {
            if self.speed.y > 0.0 {
                self.speed.y = 0.0;
                self.position.y = tile.abs_pos.y - ( TILE_SIZE + 0.1 );
                //println!("vertical: bottom_left");
                return
            }
        }
        if let Some(tile) = bottom_right  {
            if self.speed.y > 0.0 {
                self.speed.y = 0.0;
                self.position.y = tile.abs_pos.y - (TILE_SIZE + 0.1);
                //println!("vertical: bottom_right");
                return
            }
        }
    }
    pub fn check_collision_horizontal(&mut self, game_map: &HashMap<(u8, u8), Tile>) {
        let top_left = game_map.get(
            &get_tile_coords(
                self.position.y,
                self.position.x + self.speed.x + 1.0
            )
        );
        let top_right = game_map.get(
            &get_tile_coords(
                self.position.y,
                self.position.x + TILE_SIZE + self.speed.x - 1.0 
            )
        );
        let bottom_left = game_map.get(
            &get_tile_coords(
                self.position.y + TILE_SIZE,
                self.position.x + self.speed.x + 1.0
            )
        );
        let bottom_right = game_map.get(
            &get_tile_coords(
                self.position.y + TILE_SIZE,
                self.position.x + TILE_SIZE + self.speed.x + 1.0 
            )
        );
        if let Some(tile) = top_left  {
            if self.speed.x < 0.0 {
                self.speed.x = 0.0;
                self.position.x = tile.abs_pos.x + TILE_SIZE + 1.0;
                return
            }
        }
        if let Some(tile) = top_right  {
            if self.speed.x > 0.0 {
                self.speed.x = 0.0;
                self.position.x = tile.abs_pos.x - (TILE_SIZE + 1.0);
                return
            }
        }
        if let Some(tile) = bottom_left  {
            if self.speed.x < 0.0 {
                self.speed.x = 0.0;
                self.position.x = tile.abs_pos.x + TILE_SIZE + 1.0;
                //println!("bottom left {:?}", tile.tile_pos);
                //println!("bottom left {:?}", tile.abs_pos);
                return
            }
        }
        if let Some(tile) = bottom_right  {
            if self.speed.x > 0.0 {
                self.speed.x = 0.0;
                self.position.x = tile.abs_pos.x - (TILE_SIZE + 1.0);
                //println!("bottom right {:?}", tile.tile_pos);
                //println!("bottom right {:?}", tile.abs_pos);
                return
            }
        }
    }
}

