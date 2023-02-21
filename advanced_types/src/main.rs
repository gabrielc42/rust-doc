use std::fmt::{self, Error};

fn main() {
    println!("Hello, world of types! ⚡🐛❄️🌊🔥🐲");

    // new types hiding internal impl
    // People type wrap a HashMap<i32, String> that
    // stores a person's ID assoc. w/ their name

    // code using People would only interact with the public API
    // like a method to add a name string to the People collection
    // code wouldn't need to know that we assign and i32 ID to names internally

    // creating type synonyms with type aliases

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // too much
    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    //     // --snip--
    // }

    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     // --snip--
    // }

    // type alias
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi!"));

    fn takes_long_type(f: Thunk) {
        // ...
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi!"))
    }

    // Result<T, E>

    // pub trait Write {
    //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    //     fn flush(&mut self) -> Result<(), Error>;

    //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    // }

    type Result<T> = std::result::Result<T, std::io::Error>;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }

    // never type ... type that never returns: !
    fn bar() -> ! {
        // --snip--
        panic!();
    }

    // but why? recal listing 2-5, guessing number game
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // has a ! value,
                                // allowing match to return a u32 and a continue
        };

        println!("You guessed: {guess}");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // panic! has the type !
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }

    // as does loop
    // print!("4ever");

    // loop {
    //     print!("and evaaaa!");
    // }

    // dynamically sized types and the sized trait

    let s1: &str = "Hello there!"; //  str 12 bytes
    let s2: &str = "How's it going?"; // str 15 bytes
                                      // therefore, Rust needs to know how much memory to allocate for the values of a particular type
                                      // and all values of a type must use the same amount in memory
                                      // so it needs to be referenced
                                      // this is why is isn't possible to
                                      // create a variable holding a dynamically sized type

    // golden rule: always put values of dynamically sized types behind a pointer of some kind
    // Box<str>, Rc<str>, etc.
    // every trait is a dynamically sized type we can refer to by using name of trait
    // traits as trait objects:
    // &dyn Trait, Box<dyn Trait>, Rc<dyn Trait>, ...

    // Sized trait
    fn generically_generic<T>(t: T) {
        // ...
    }

    // is actually:
    fn is_actually_generic<T: Sized>(t: T) {
        // ...
    }

    // generic functions will work only on types that
    // have a known size at compile time
    // T may or may not be Sized
    fn generic<T: ?Sized>(t: &T) {
        // ...
    }
}
