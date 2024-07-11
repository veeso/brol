use std::cmp::{Eq, Ord};
use std::error::Error;
use std::ops::Add;
use std::str::FromStr;

fn main() {
    let range = parse_range::<u64>("0-3").unwrap();
    assert_eq!(range, vec![0, 1, 2, 3]);
    println!("{:?}", range);
    let range = parse_range::<u64>("0,1,2,3").unwrap();
    assert_eq!(range, vec![0, 1, 2, 3]);
    println!("{:?}", range);

    let range = parse_range_alt::<u64>("0-3").unwrap();
    assert_eq!(range, vec![0, 1, 2, 3]);
    println!("{:?}", range);
    let range = parse_range_alt::<u64>("0,1,2,3").unwrap();
    assert_eq!(range, vec![0, 1, 2, 3]);
    println!("{:?}", range);
}

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

/// Parse a range string to a vector of usize
///
/// # Arguments
/// - range_str: &str - the range string to parse
///
/// # Returns
/// - Result<Vec<T>, anyhow::Error> - the parsed range
///
/// # Example
///
/// ```rust
/// let range: Vec<u64> = parse_range::<u64>("0-3").unwrap();
/// assert_eq!(range, vec![0, 1, 2]);
///
/// let range: Vec<u64> = parse_range::<u64>("0,1,2,3").unwrap();
/// assert_eq!(range, vec![0, 1, 2, 3]);
/// ```
fn parse_range<T>(range_str: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr + Add<Output = T> + Eq + Ord + Unit + Copy,
{
    // parse both format: 0-3 or 0,1,2,3
    if range_str.contains('-') {
        let mut range = range_str.split('-');
        let start = range
            .next()
            .ok_or_else(|| "invalid range: start token not found")?;
        let end = range
            .next()
            .ok_or_else(|| "invalid range: end token not found")?;
        let start = start
            .parse::<T>()
            .map_err(|_| "invalid range: start is not a number")?;
        let end = end
            .parse::<T>()
            .map_err(|_| "invalid range: end is not a number")?;

        let mut range = Vec::new();
        let mut x = start;
        while x <= end {
            range.push(x);
            x = x + T::unit();
        }

        Ok(range)
    } else {
        let range = range_str
            .split(',')
            .map(|s| {
                s.parse::<T>()
                    .map_err(|_| "invalid range values: not a number")
            })
            .collect::<Result<Vec<T>, _>>()?;
        Ok(range)
    }
}

fn parse_range_alt<T>(range_str: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr + TryInto<isize> + TryFrom<isize>,
{
    // parse both format: 0-3 or 0,1,2,3
    if range_str.contains('-') {
        let mut range = range_str.split('-');
        let start = range
            .next()
            .ok_or_else(|| "invalid range: start token not found")?;
        let end = range
            .next()
            .ok_or_else(|| "invalid range: end token not found")?;
        let start = start
            .parse::<isize>()
            .map_err(|_| "invalid range: start is not a number")?;
        let end = end
            .parse::<isize>()
            .map_err(|_| "invalid range: end is not a number")?;

        let range = (start..=end).collect::<Vec<isize>>();
        let mut t_range = Vec::with_capacity(range.len());
        for x in range {
            if let Ok(x) = x.try_into() {
                t_range.push(x);
            } else {
                return Err("invalid range values: conversion error".into());
            }
        }
        Ok(t_range)
    } else {
        let range = range_str
            .split(',')
            .map(|s| {
                s.parse::<T>()
                    .map_err(|_| "invalid range values: not a number")
            })
            .collect::<Result<Vec<T>, _>>()?;
        Ok(range)
    }
}
