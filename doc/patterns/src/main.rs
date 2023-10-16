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
    let mut msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match_msg(msg);
    msg = Message::Quit;
    match_msg(msg);
    msg = Message::Move { x: 1, y: 2 };
    match_msg(msg);
    msg = Message::Write("lalala".to_string());
    match_msg(msg);

    // nested structs and enums

    msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green{g}, and blue{b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignoring an entire value w/ _
    foo(3, 4);

    // ignoring parts of a value w/ a nested _

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value (╯•﹏•╰)");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (4, 36, 823, 12, 0, 2);

    match numbers {
        (first, _, third, _, fifth, _) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // ignoring unused var by starting name w/ _
    let _x = 0;
    let y = 1;

    // will give error because _s is value-binded:
    // let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    //     println!("found a string over here");
    // }

    // println!("{:?}", s);

    let s = Some(String::from("hello!"));

    if let Some(_) = s {
        println!("found over a string over for real this time");
    }

    println!("{:?}", s);

    // ignoring remaining parts of a value w/ ..
    match Origin {
        PointTwo { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            // (.., second, ..) doesn't compile, '..' only once!
            println!("Some numbers from earlier: {first}, {last}");
        }
    }

    // extra conditionals w/ match guards

    let mut num = Some(4);
    match_guard(num);
    num = Some(5);
    match_guard(num);

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50! （・⊝・∞）"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y} ... ｡^‿^｡", x);

    let x = 4;
    let y = false;

    match x {
        // (4 | 5 | 6) if y => ... rather than 4 | 5 | (6 if y) => ...
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ bindings
    let mut msg = MessageTwo::Hello { id: 5 };
    match_msg_two(msg);

    msg = MessageTwo::Hello { id: 2 };
    match_msg_two(msg);

    msg = MessageTwo::Hello { id: 10 };
    match_msg_two(msg);
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
            print!("text message: {text} o_O \n");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(" ~ Change the color to red {r}, green {g}, and blue {b} ~ ",);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(" ( Change the color to hue {h}, saturation {s}, value {v} ) ",);
        }
    }
}

// ignoring an entire value w/ _
fn foo(_: i32, y: i32) {
    println!("Y parameter: {}", y);
}

// ignoring remaining parts of a value w/ ..
struct PointTwo {
    x: i32,
    y: i32,
    z: i32,
}

const Origin: PointTwo = PointTwo { x: 0, y: 0, z: 0 };

// extra conditionals w/ match guards

fn match_guard(num: Option<i32>) {
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

// @ bindings
enum MessageTwo {
    Hello { id: i32 },
}

fn match_msg_two(msg: MessageTwo) {
    match msg {
        MessageTwo::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {} (￣个￣)", id_variable),
        MessageTwo::Hello { id: 10..=12 } => {
            println!("Found an id in another range  ٩(｡•́‿•̀｡)۶")
        }
        MessageTwo::Hello { id } => println!("Found some other id: {} °˖✧◝(⁰▿⁰)◜✧˖°", id), // id: 2 for example
    }
}
