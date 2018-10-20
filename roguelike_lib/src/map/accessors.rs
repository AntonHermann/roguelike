//! Accessors to map: get() and index()

use super::{Map, Cell};

impl<T> Map<T> {
    /// Safe getter
    pub fn get(&self, cell: Cell) -> Option<&T> {
        if self.is_valid(cell) {
            Some(&self[cell])
        } else {
            None
        }
    }
    /// Safe mutable getter
    pub fn get_mut(&mut self, cell: Cell) -> Option<&mut T> {
        if self.is_valid(cell) {
            Some(&mut self[cell])
        } else {
            None
        }
    }
    /// Is this a valid cell?
    pub fn is_valid(&self, (x, y): Cell) -> bool {
        x >= 0 && x < self.width &&
        y >= 0 && y < self.height
    }
}

impl<T> std::ops::Index<Cell> for Map<T> {
    type Output = T;

    fn index(&self, (x, y): Cell) -> &Self::Output {
        assert!(x < self.width);
        let id = x + y * self.width;
        &self.field[id as usize]
    }
}
impl<T> std::ops::IndexMut<Cell> for Map<T> {
    fn index_mut(&mut self, (x, y): Cell) -> &mut Self::Output {
        assert!(x < self.width);
        let id = x + y * self.width;
        &mut self.field[id as usize]
    }
}