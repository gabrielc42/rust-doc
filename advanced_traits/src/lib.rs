struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 1. Q: why not define Iterator w/ generics?
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
// 1. A: when we perform next method on Counter, we would have to provide type annotations to
// indicate which implementation of Iterator we want to use.

// We don't have to annotate types with associated types because we cannot
// implement a trait on a type multiple times
// can only choose what the type of Item will be once, because there can only
// be one impl Iterator for Counter, we don't have to specify u32 values everywhere

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// above: implementing Add trait on Millimeters to add Millimeters to Meters
// default type parameters in two main ways:
// extend a type w/out breaking existing code
// allow customization in specific cases most users won't need
