#![allow(unused)]

use std::fmt;

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

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // trait w/ associated function and a type with
    // an associated function of the same name that also implements the trait
    println!("A baby dog is called a {}", Dog::baby_name());

    // rust won't know which impl, because no self parameter and could be other types that impl Animal trait
    // println!("A baby dog is called a {}", Animal::baby_name());

    // fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);

    // supertraits to require one trait's functionality within another trait
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // if no ::Display on fmt, we'd get an error 'required but not implemented'

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // newtype pattern to implement external traits on external types
}
