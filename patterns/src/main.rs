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
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println! {"Current location: ({}, {})", x, y};
}
