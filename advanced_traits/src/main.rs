fn main() {
    println!("Hello, world of traits!");

    // specifying placeholder types in trait defintions w/ assoicated types
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // ...in lib.rs...

    // default generic type parameters and operator overloading
}
