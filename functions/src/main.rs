fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;

    println!("Hello, world!");

    let y = {
        // expression
        let x = 4;
        x + 1 // if a semicolon is added to an end of an expression, it will turn into a statement and not return a value
    };
    println!("The value of y is: {y}");
    !another_function(5);

    let w = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("Another function. The value of x is: {x}");
}

fn five() -> i32 {
    5
}
