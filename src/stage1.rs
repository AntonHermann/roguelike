use roguelike_lib::*;
use std::fmt;
use crate::Result;
use termion::event::Key;

pub struct Stage1 {
    level: Level,
}
impl Stage1 {
    pub fn new() -> Self {
        let level = Level::generate_new("Stage1");
        Self {
            level,
        }
    }
    pub fn handle_key(&mut self, key: termion::event::Key) -> Result<()> {
        let dir = match key {
            Key::Up     => Dir::Up   ,
            Key::Right  => Dir::Right,
            Key::Left   => Dir::Left ,
            Key::Down   => Dir::Down ,
            _ => return Ok(()),
        };
        if !self.level.player_move(dir) {
            // TODO: Any feedback if user couln't move?
        }
        Ok(())
    }
}
impl fmt::Display for Stage1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use termion::color;

        for di in self.level.iter_display() {
            match di {
                DisplayItem::Player => {
                    write!(f, "@")?
                },
                DisplayItem::Tile(t) => write!(f, "{}", t)?,
                DisplayItem::LineBreak => write!(f, "\n\r")?,
            }
        }
        Ok(())
    }
}