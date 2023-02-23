// function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    println!("Hello, world of functions and closures!");

    // function pointers
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // closure defined inline or a named function
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numberz = vec![1, 2, 3];
    let list_of_stringz: Vec<String> = list_of_numberz.iter().map(ToString::to_string).collect();

    // initializer functions as function pointers that implement closure traits
    // specify initializer functions as arguments for methods that take closures
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // returning closures
    // below doesn't compile
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
