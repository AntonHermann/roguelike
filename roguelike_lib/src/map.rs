#![allow(dead_code)]
#![deny(missing_docs)]
//! A 2D Map with generic field type

#[cfg(test)]
mod tests;
pub mod neighbours;
pub mod iterators;
pub mod constructors;
pub mod accessors;
pub mod manipulations;

/// An (x,y) index to a cell
pub type Cell = (i32, i32);

use std::fmt;

#[derive(Debug)]
/// A 2D Map of Field Type T
pub struct Map<T> {
    width: i32,
    height: i32,
    field: Vec<T>,
}

// === GENERAL PROPERTIES === //
impl<T> Map<T> {
    /// The height of the map
    pub fn height(&self) -> i32 {
        self.height
    }
    /// The width of the map
    pub fn width(&self) -> i32 {
        self.width
    }
}

// === DISPLAY === //
impl<T: fmt::Display> fmt::Display for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for v in self.iter_row(y) {
                write!(f, "{}", v)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}