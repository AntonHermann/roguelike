//! Access to the neighbours of a Cell

use super::Map;

impl<T> Map<T> {
    /// Returns all adjacent cells
    pub fn get_neighbours<'a>(&'a self, x: i32, y: i32) -> Neighbours<'a, T> {
        let mut neighbours: [[Option<&'a T>; 3]; 3] = [[None;3];3];
        for (iy,dy) in [-1,0,1].into_iter().enumerate() {
            for (ix,dx) in [-1,0,1].into_iter().enumerate() {
                let _x = x as i32 + dx;
                let _y = y as i32 + dy;
                if _x >= 0 && _y >= 0 {
                    neighbours[iy][ix] = self.get((_x, _y))
                }
            }
        }
        Neighbours {
            neighbours,
        }
    }
}
/// The neighbours of a specific cell
/// 
/// NW N NE
///  W *  E
/// SW S SE
#[derive(Debug)]
pub struct Neighbours<'a, T> {
    neighbours: [[Option<&'a T>; 3]; 3],
}
impl<'a, T> Neighbours<'a, T> {
    const N: usize = 0;
    const C: usize = 1;
    const S: usize = 2;
    const W: usize = 0;
    const E: usize = 2;

    /// north west neighbour
    pub fn nw(&self) -> Option<&'a T> {
        self.neighbours[Self::N][Self::W]
    }
    /// north neighbour
    pub fn n(&self) -> Option<&'a T> {
        self.neighbours[Self::N][Self::C]
    }
    /// north east neighbour
    pub fn ne(&self) -> Option<&'a T> {
        self.neighbours[Self::N][Self::E]
    }
    /// west neighbour
    pub fn w(&self) -> Option<&'a T> {
        self.neighbours[Self::C][Self::W]
    }
    /// the cell itself
    pub fn c(&self) -> Option<&'a T> {
        self.neighbours[Self::C][Self::C]
    }
    /// east neighbour
    pub fn e(&self) -> Option<&'a T> {
        self.neighbours[Self::C][Self::E]
    }
    /// south west neighbour
    pub fn sw(&self) -> Option<&'a T> {
        self.neighbours[Self::S][Self::W]
    }
    /// south neighbour
    pub fn s(&self) -> Option<&'a T> {
        self.neighbours[Self::S][Self::C]
    }
    /// south east neighbour
    pub fn se(&self) -> Option<&'a T> {
        self.neighbours[Self::S][Self::E]
    }
}