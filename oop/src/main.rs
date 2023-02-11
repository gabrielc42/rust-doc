use crate::blog::blog;
mod blog;
// use oop::Draw;
// use oop::{Button, Screen};
use oop::Post;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// impl Draw for SelectBox {
//     fn draw(&self) {}
// }

fn main() {
    // let mut post = Post::new();

    // post.add_text("I ate a salad for lunch today");
    // let post = post.request_review();
    // let post = post.approve();

    // assert_eq!("I ate a salad for lunch today", post.content());

    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))], // passing something wrong
    //                                                     // implement Draw on String so that Screen is able to call Draw
    // };
    // screen.run();

    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No"),
    //             ],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("OK"),
    //         }),
    //     ],
    // };
    // screen.run();

    blog()
}

// With the state pattern,
// the Post methods and the places we use Post don’t need match expressions,
// and to add a new state, we would only need to add a new struct
// and implement the trait methods on that one struct.
