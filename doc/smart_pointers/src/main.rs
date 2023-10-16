#![allow(dead_code, unused_variables)]

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// could change Cons def to hold ref, then specify lifetime
// case for this example, but not every scenario
// instead: Rc<T> in place of Box<T>
// each Cons variant will now hold a value and an Rc<T>
// pointing to a List

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

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

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// ref from child to parent
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
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
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program
    // for reading only. If Rc<T> allowed you to have multiple mutable references too, you might violate one
    // of the borrowing rules discussed in Chapter 4: multiple mutable borrows to
    // the same place can cause data races and inconsistencies

    //multiple owners of mutable data by combining Rc<T> and RefCell<T>
    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    //reference cycles leaking memory
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // // Uncomment the next line to see that we have a cycle;
    // // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    //tree data structure
    // let leaf = Rc::new(Node {
    //     value: 3,
    //     children: RefCell::new(vec![]),
    // });

    // let branch = Rc::new(Node {
    //     value: 5,
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });

    //from child to parent
    // let leaf = Rc::new(Node {
    //     value: 3,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![]),
    // });

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // let branch = Rc::new(Node {
    //     value: 5,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });

    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    //changes to strong_count and weak_count
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

//Rust does deref coercion when it finds types and trait implementations in three cases:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

// immutable references will never coerce to mutable references
// Converting an immutable reference to
// a mutable reference would require that the initial immutable reference is
// the only immutable reference to that data, but the borrowing rules don’t guarantee that

// Rc<T> enables multiple owners of the same data;
// Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time;
// Rc<T> allows only immutable borrows checked at compile time;
// RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

// automated tests, code reviews, and other software development practices to minimize reference cycles
// also, reorganizing your data structures so that some references express ownership and some references don’t.
// As a result, you can have cycles made up of some ownership relationships and some non-ownership relationships,
// and only the ownership relationships affect whether or not a value can be dropped.
