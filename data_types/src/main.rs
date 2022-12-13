//four scalar types: integers, floating-point, Boolean, characters
//signed or unsigned means having a - or not
fn main() {
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; //w/ explicit type annotation

    let c = 'z';
    let z: char = 'Z'; //w/ explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //i32 type of each element, 5 elements total
    let a: [3; 5]; //5 elements of 3
    let w = [1,2,3,4,5];
    let first = w[0];
}

}
