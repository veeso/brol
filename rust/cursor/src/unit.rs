/// A trait for types that have a unit value.
///
/// E.g. 1 for integers, 1.0 for floats, etc.
pub trait Unit {
    fn unit() -> Self;
}

/// Implement One for common numeric types.
macro_rules! impl_one_for_numeric {
    ($($t:ty)*) => ($(
        impl Unit for $t {
            fn unit() -> Self {
                1
            }
        }
    )*)
}

impl_one_for_numeric!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);

/// Implement One for common float types.
macro_rules! impl_one_for_floats {
    ($($t:ty)*) => ($(
        impl Unit for $t {
            fn unit() -> Self {
                1.0
            }
        }
    )*)
}

impl_one_for_floats!(f32 f64);
