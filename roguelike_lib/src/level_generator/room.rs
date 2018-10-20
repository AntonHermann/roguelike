#[derive(Debug)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub center: (i32, i32),
}
impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            x2: x + width,
            y2: y + height,
            width,
            height,
            center: (x + (width / 2), y + (height / 2)),
        }
    }
    pub fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x &&
        self.y <= other.y2 && self.y2 >= other.y
    }
    pub fn center(&self) -> (i32, i32) {
        self.center
    }
}