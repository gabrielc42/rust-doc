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
    use std::ops::Add as AddGeneric;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl AddGeneric for Point {
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

    // default generic type within Add
    trait Add<Rhs = Self> {
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
    }

    // fully qualified syntax for disambiguation: calling methods with same name
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking 📞🐼")
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up! ⬆️");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously* 🐣")
        }
    }

    let person = Human;
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
