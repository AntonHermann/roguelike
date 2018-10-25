use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn ch(&self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall  => '#',
        }
    }
    /// Can someone walk on this tile?
    pub fn passable(&self) -> bool {
        match self {
            Tile::Floor => true,
            Tile::Wall  => false,
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ch())
    }
}