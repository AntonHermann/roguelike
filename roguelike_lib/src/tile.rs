use std::fmt;

#[derive(Debug, Clone)]
pub struct Tile {
    ty: TileType,
}

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
}

impl Tile {
    pub const FLOOR: Self = Self { ty: TileType::Floor };
    pub const WALL: Self = Self { ty: TileType::Wall };
    pub fn ch(&self) -> &str {
        match self.ty {
            TileType::Floor => " ",
            TileType::Wall => "|",
        }
    }
    /// Can someone walk on this tile?
    pub fn passable(&self) -> bool {
        match self.ty {
            TileType::Floor => true,
            TileType::Wall  => false,
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ch())
    }
}