//! Map manipulation methods

use super::Map;

impl<T: Clone> Map<T> {
    /// Fill a rect with item
    /// 
    /// Panics if x/y are out of bounds
    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, item: &T) {
        assert!(x >= 0, "x has to be >= 0");
        assert!(y >= 0, "y has to be >= 0");
        assert!(x+w < self.width , "x+w has to be < width (x:{}+w:{} < width:{})", x, w, self.width);
        assert!(y+h < self.height, "y+h has to be < height (y:{}+h:{} < height:{})", y, h, self.height);
        for col in x..x+w {
            for row in y..y+h {
                self[(col, row)] = item.clone();
            }
        }
    }
}