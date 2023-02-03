// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// could change Cons def to hold ref, then specify lifetime
// case for this example, but not every scenario
// instead: Rc<T> in place of Box<T>
// each Cons variant will now hold a value and an Rc<T>
// pointing to a List

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // box<t> and deref trait
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let b = Box::new(5);
    // println!("b = {}", b);

    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);

    // let m = MyBox::new(String::from("Rust"));
    // hello(&(*m)[..]);

    // drop trait
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");

    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };
    // println!("CustomSmartPointer created.");
    // // c.drop(); // destructor analogous to constructor (cleans up an instance)
    // // double free error
    // // rather,
    // drop(c);
    // println!("CustomSmartPointer dropped before the end of main.");

    // Rc<T>
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program
    // for reading only. If Rc<T> allowed you to have multiple mutable references too, you might violate one
    // of the borrowing rules discussed in Chapter 4: multiple mutable borrows to
    // the same place can cause data races and inconsistencies
}

//Rust does deref coercion when it finds types and trait implementations in three cases:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

// immutable references will never coerce to mutable references
// Converting an immutable reference to
// a mutable reference would require that the initial immutable reference is
// the only immutable reference to that data, but the borrowing rules don’t guarantee that
