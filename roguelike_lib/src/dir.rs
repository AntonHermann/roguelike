#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Right,
    Left,
}
use self::Dir::*;
impl Dir {
    /// Direction interpretet as relative movement
    pub fn as_rel_movement(&self) -> (i32, i32) {
        match self {
            Up    => ( 0, -1),
            Down  => ( 0,  1),
            Right => ( 1,  0),
            Left  => (-1,  0),
        }
    }
}