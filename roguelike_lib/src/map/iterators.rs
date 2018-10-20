//! Iterators over Map

use super::Map;

impl<T> Map<T> {
    /// Returns an iterator over the fields
    /// 
    /// Doesn't make guarantees about ordering
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> {
        self.field.iter()
    }
    /// Returns an iterator that allows modifying each value.
    /// 
    /// Doesn't make guarantees about ordering
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> {
        self.field.iter_mut()
    }
    /// Returns an iterator over the fields of a certain row
    pub fn iter_row<'a>(&'a self, y: i32) -> RowIter<'a, T> {
        let left  = (y * self.width) as usize;
        let right = (y * self.width + self.width) as usize;
        RowIter {
            inner: &self.field[left..right],
            x: 0,
        }
    }
    // /// Returns an iterator over the fields of a certain row
    // pub fn iter_row_mut<'a>(&'a mut self, y: i32) -> RowIterMut<'a, T> {
    //     let left = y * self.width;
    //     let right = y * self.width + self.width;
    //     RowIterMut {
    //         inner: &mut self.field[left..right],
    //         x: 0,
    //     }
    // }
    /// Returns an iterator over the fields of a certain column
    pub fn iter_col<'a>(&'a self, x: i32) -> ColIter<'a, T> {
        ColIter {
            inner: &self,
            x,
            y: 0,
        }
    }
    // /// Returns an iterator over the fields of a certain column
    // pub fn iter_col_mut<'a>(&'a mut self, x: i32) -> ColIterMut<'a, T> {
    //     ColIterMut {
    //         inner: &mut self,
    //         x,
    //         y: 0,
    //     }
    // }
}

/// An Iterator over the rows of a Map
pub struct RowIter<'a, T> {
    inner: &'a [T],
    x: i32,
}
/// An Iterator over the columns of a Map
pub struct ColIter<'a, T> {
    inner: &'a Map<T>,
    x: i32,
    y: i32,
}
impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.inner.len() as i32 {
            let ret = &self.inner[self.x as usize];
            self.x += 1;
            Some(ret)
        } else {
            None
        }
    }
}
impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.inner.height {
            let ret = &self.inner[(self.x, self.y)];
            self.y += 1;
            Some(ret)
        } else {
            None
        }
    }
}
/* // === MUT ITERATORS (not working) === //
    // /// An mutable Iterator over the rows of a Map
    // pub struct RowIterMut<'a, T> {
    //     inner: &'a mut [T],
    //     x: i32,
    // }
    // /// An mutable Iterator over the columns of a Map
    // pub struct ColIterMut<'a, T: 'a> {
    //     inner: &'a mut Map<T>,
    //     x: i32,
    //     y: i32,
    // }
    // impl<'a, T> Iterator for RowIterMut<'a, T> {
    //     type Item = &'a mut T;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.x < self.inner.len() {
    //             let ret = &mut self.inner[self.x];
    //             self.x += 1;
    //             Some(&mut ret)
    //         } else {
    //             None
    //         }
    //     }
    // }
    // impl<'a, T> Iterator for ColIterMut<'a, T> {
    //     type Item = &'a mut T;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.y < self.inner.height {
    //             let ret: &'a mut T = &mut self.inner[(self.x, self.y)];
    //             self.y += 1;
    //             Some(ret)
    //         } else {
    //             None
    //         }
    //     }
    // }
*/