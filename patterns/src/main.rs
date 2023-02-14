#![allow(dead_code, unused_variables)]

fn main() {
    // if let, else if, else if let, else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "25".parse();

    if let Some(color) = favorite_color {
        println!("Your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Purple as the background color");
        } else {
            println!("Orange as the background color");
        }
    } else {
        println!("Blue as the background color");
    }

    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let
    let (x, y, z) = (1, 2, 3); // left right amounts must be same

    // function params
    let point = (2, 3);
    print_coordinates(&point);

    // let Some(x) = some_option_value; // doesn't compile, pattern is refutable (may be a None value)
    // can instead type 'if let'
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }
    // however,
    if let x = 5 {
        println! {"{}", x};
    }; // is refutable and suggests 'let' only

    // matching literals

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named vars

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    // will be a new y var, binding to inner value of Some in x

    println!("at the end: x = {:?}, y = {y}", x);

    // multiple patterns

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values ..=
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // range char values

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("somethin' else i reckon..."),
    }

    // destructuring to break apart values
    // structs

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p; // rather than x: a, y: b do x, y y'know
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // enums
    // let mut msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match_msg(msg);
    msg = Message::Quit;
    match_msg(msg);
    msg = Message::Move { x: 1, y: 2 };
    match_msg(msg);
    msg = Message::Write("lalala".to_string());
    match_msg(msg);

    // nested structs and enums
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green{g}, and blue{b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println! {"Current location: ({}, {})", x, y};
}

// struct destruct
struct Point {
    x: i32,
    y: i32,
}

// enum destruct (and nested structs)
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    // ChangeColor(i32, i32, i32),
    ChangeColor(Color),
}

fn match_msg(msg: Message) {
    match msg {
        Message::Quit => {
            println!("[[The Quit variant has no data to destructure!]]")
        }
        Message::Move { x, y } => {
            println!(" ... Move in the x direction: [{x}] ... and in the y direction: [{y}] ... ");
        }
        Message::Write(text) => {
            print!("text message: {text} o_O");
        }
        Message::ChangeColor(r, g, b) => {
            println!(" ~ Change the color to red {r}, green {g}, and blue {b} ~ ",)
        }
    }
}
