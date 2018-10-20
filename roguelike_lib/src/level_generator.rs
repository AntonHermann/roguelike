use crate::Map;
use crate::Tile;

mod room;
use self::room::Room;

use rand::{self, Rng};
// use std::fmt;

use log::*;

#[derive(Debug)]
pub struct LevelGenerator {
    map: Map<Tile>,
    rooms: Vec<Room>,
    rng: rand::StdRng,
}
impl LevelGenerator {
    pub fn new(width: i32, height: i32, rng: rand::StdRng) -> Self {
        Self {
            map: Map::new_from_item(width, height, Tile::WALL),
            rooms: Vec::new(),
            rng,
        }
    }
    pub fn place_rooms(&mut self) {
        // FIXME: this has to be somehow adaptive
        let max_rooms = 5;
        let min_room_width = 4;
        let max_room_width = 8;
        let min_room_height = 4;
        let max_room_height = 8;

        let map_width  = self.map.width()  as i32;
        let map_height = self.map.height() as i32;

        info!("max_rooms:{}, room_w:{}-{}, room_h:{}-{}, map_dim:{}*{}",
            max_rooms,
            min_room_width , max_room_width ,
            min_room_height, max_room_height,
            map_width, map_height
        );
        for i in 0..max_rooms {
            let mut x = self.rng.gen_range(0, map_width );
            let mut y = self.rng.gen_range(0, map_height);

            let width  = self.rng.gen_range(min_room_width , max_room_width );
            let height = self.rng.gen_range(min_room_height, max_room_height);

            debug!("{}: x:{},y:{},w:{},h:{}", i, x, y, width, height);

            // if it's off the board, shift it back in again
            if x + width >= map_width {
                x = map_width - width;
            }
            if y + height >= map_height {
                y = map_height - height;
            }
            // if this didn't fix it, just continue
            if  x + width >= map_width || y + height >= map_height {
                debug!("    x:{},y:{}, continue", x, y);
                continue;
            }

            debug!("    x:{},y:{}", x, y);

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            // check all other rooms we've placed to see if this one
            // collides with them
            for other_room in &self.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break;
                }
            }
            debug!("    collision: {}", collides);

            // if the new room doesn't collide, add it to the level
            if !collides {
                self.add_room(room);
            }
        }
    }
    fn add_room(&mut self, room: Room) {
        self.map.fill_rect(room.x, room.y, room.width, room.height, &Tile::FLOOR);
        self.rooms.push(room);
    }
    pub fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }
    pub fn get_map(self) -> Map<Tile> {
        self.map
    }
}

// impl fmt::Display for Level {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for row in 0..self.map.height() {
//             for col in 0..self.map.width() {
//                 write!(f, "{}", self.map[(col, row)])?
//             }
//             write!(f, "\n")?
//         }

//         Ok(())
//     }
// }