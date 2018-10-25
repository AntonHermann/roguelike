use crate::*;
use crate::level_generator::LevelGenerator;

pub const SCREEN_WIDTH:  i32 = 80;
pub const SCREEN_HEIGHT: i32 = 24;

#[derive(Debug)]
pub struct Level {
    map: Map<Tile>,
    player: Option<Cell>,
}
impl Level {
    pub fn generate_new(seed: &str) -> Self {
        let rng = crate::rand_utils::create_rng(seed);
        let mut level_gen = LevelGenerator::new(
            SCREEN_WIDTH, SCREEN_HEIGHT, rng);
        level_gen.place_rooms();
        let player = Some({
            let rooms = level_gen.rooms();
            let room = &rooms[rooms.len() / 2];
            room.center()
        });
        let map = level_gen.get_map();

        Self {
            map,
            player,
        }
    }
    pub fn generate_blank() -> Self {
        let width  = SCREEN_WIDTH;
        let height = SCREEN_HEIGHT;
        let player = Some((width/2, height/2));
        let map = Map::new_with_border(
            width, height, Tile::Floor, Tile::Wall
        );

        Self {
            map,
            player,
        }
    }
    pub fn map(&self) -> &Map<Tile> {
        &self.map
    }
    pub fn player(&self) -> Option<&Cell> {
        self.player.as_ref()
    }
    pub fn can_move(&self, src: Cell, dir: Dir) -> bool {
        let dir_rel = dir.as_rel_movement();
        let dest = (src.0 + dir_rel.0, src.1 + dir_rel.1);
        self.map.get(dest).map(|tile| tile.passable()) == Some(true)
    }
    /// Try to move the player in `dir`
    /// Returns true if successfull
    pub fn player_move(&mut self, dir: Dir) -> bool {
        if let Some(player) = self.player {
            let dir_rel = dir.as_rel_movement();
            let dest = (player.0 + dir_rel.0, player.1 + dir_rel.1);
            if self.can_move(player, dir) {
                self.player = Some(dest);
                return true
            }
        }
        false
    }
    pub fn iter_display<'a>(&'a self) -> IterDisplay<'a> {
        IterDisplay {
            inner: &self,
            x: 0,
            y: 0,
        }
    }
}
pub struct IterDisplay<'a> {
    inner: &'a Level,
    x: i32,
    y: i32,
}
pub enum DisplayItem<'a> {
    Player,
    Tile(&'a Tile),
    LineBreak,
}
impl<'a> Iterator for IterDisplay<'a> {
    type Item = DisplayItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let width  = self.inner.map.width();
        let height = self.inner.map.height();

        // Iteration is over
        if self.y == height {
            return None;
        }
        // End of line
        if self.x == width {
            self.x = 0;
            self.y += 1;
            return Some(DisplayItem::LineBreak);
        }

        let cell = (self.x, self.y);
        let ret = if self.inner.player == Some(cell) {
            DisplayItem::Player
        } else {
            DisplayItem::Tile(&self.inner.map[cell])
        };
        self.x += 1;
        Some(ret)
    }
}