#[derive(Debug)]
pub enum Dir {
    NW, N, NE,
     W,     E,
    SW, S, SE,
}
use self::Dir::*;
impl Dir {
    /// Direction interpretet as relative movement
    pub fn as_rel_movement(&self) -> (i32, i32) {
        match self {
            NW => (-1, -1),
            N  => ( 0, -1),
            NE => ( 1, -1),

            E  => ( 1,  0),
            W  => (-1,  0),

            SW => (-1,  1),
            S  => ( 0,  1),
            SE => ( 1,  1),
        }
    }
}