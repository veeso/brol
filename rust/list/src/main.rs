#![feature(step_trait)]

use std::iter::Step;
use std::ops::Range;

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(Box<Item<T>>),
}

#[derive(Debug)]
struct Item<T> {
    val: T,
    next: List<T>,
}

impl<T> List<T> {
    pub fn len(&self) -> usize {
        self.reduce(0, |_, acc| acc + 1)
    }

    fn reduce<F, U>(&self, acc: U, reducer: F) -> U
    where
        F: Fn(&List<T>, U) -> U,
    {
        match self {
            Self::Nil => acc,
            Self::Cons(item) => item.next.reduce(reducer(self, acc), reducer),
        }
    }
}

impl<T> From<Range<T>> for List<T>
where
    T: PartialOrd<T> + Step + std::fmt::Debug,
{
    fn from(mut range: Range<T>) -> Self {
        if range.is_empty() {
            Self::Nil
        } else {
            let val = range.start.clone();
            let next = match range.clone().nth(1) {
                None => Self::Nil,
                Some(start) => List::from(start..range.end),
            };
            Self::Cons(Box::new(Item { val, next }))
        }
    }
}

fn main() {
    let l = List::from(0..10);
    println!("{:?}", l);

    println!("list len: {}", l.len());
}
