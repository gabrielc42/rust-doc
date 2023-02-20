#![allow(unused)]

fn main() {
    println!("Hello, world of traits!");

    // specifying placeholder types in trait defintions w/ assoicated types
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // ...in lib.rs...

    // default generic type parameters and operator overloading
    // overloading + operator to add two Point instances together
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // default gerneric type within Add
    trait Add<Rhs = Self> {
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
    }
}
