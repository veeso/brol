mod unit;

use std::ops::{Add, Rem};

use self::unit::Unit;

/// A cursor that can be used to iterate over a collection of items
#[derive(Clone)]
pub struct Cursor<T>
where
    T: Add<Output = T> + Clone + PartialEq + Unit + Default + Rem<Output = T>,
{
    value: T,
    limit: T,
}

impl<T> Cursor<T>
where
    T: Add<Output = T> + Clone + Rem<Output = T> + PartialEq + Unit + Default,
{
    /// Create a new cursor
    pub fn new(limit: T) -> Self {
        Cursor {
            value: T::default(),
            limit,
        }
    }

    /// Move the cursor to the next position
    pub fn step(&mut self) {
        self.value = (self.value.clone() + T::unit()) % self.limit.clone();
    }

    /// Get the current value of the cursor
    pub fn value(&self) -> T {
        self.value.clone()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_should_wrap_cursor() {
        let mut cursor = Cursor::new(3usize);
        cursor.step();
        assert_eq!(cursor.value, 1);
        cursor.step();
        assert_eq!(cursor.value, 2);
        cursor.step();
        assert_eq!(cursor.value, 0);
    }
}
