//! Different ways to construct a Map

use super::Map;

// === CONSTRUCTORS === //
impl<T: Default> Map<T> {
    /// Create a new Map, initializing each field by using the default value
    pub fn new_from_default(width: i32, height: i32) -> Self {
        Self::new_from_closure(width, height, |_x, _y| Default::default())
    }
}
impl<T: Clone> Map<T> {
    /// Create a new Map, initializing each field by cloning the given item
    pub fn new_from_item(width: i32, height: i32, item: T) -> Self {
        Self {
            width,
            height,
            field: vec![item; (width * height) as usize],
        }
    }
    /// Create a new map, filling the center with one item
    /// and the border with another
    /// 
    /// Panics if width or height are < 1
    pub fn new_with_border(width: i32, height: i32, center: T, border: T) -> Self {
        assert!(width > 1 && height > 1);

        let mut map = Self::new_from_item(width, height, center);

        // left and right border
        for &x in &[0, width - 1] {
            for y in 0..height {
                map[(x, y)] = border.clone();
            }
        }
        // top and bottom border
        for &y in &[0, height - 1] {
            for x in 0..width {
                map[(x, y)] = border.clone();
            }
        }

        map
    }
}
impl<T> Map<T> {
    /// Create a new Map, initializing each field by calling a closure
    pub fn new_from_closure<F>(width: i32, height: i32, cl: F) -> Self
    where
        F: Fn(i32, i32) -> T
    {
        let mut field = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                field.push(cl(x, y));
            }
        }
        Self {
            width,
            height,
            field,
        }
    }
}