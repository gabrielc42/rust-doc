fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{}", THREE_HOURS_IN_SECONDS);
    let x = 5;
    let x = x + 1;

    let spaces = "   ";
    let spaces = spaces.len();

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}, spaces variable equals: {spaces}");
}
