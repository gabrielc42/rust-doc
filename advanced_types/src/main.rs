fn main() {
    println!("Hello, world of types! ⚡🐛❄️🌊🔥🐲");

    // new types hiding interal impl
    // People type wrap a HashMap<i32, String> that
    // stores a person's ID assoc. w/ their name

    // code using People would only interact with the public API
    // like a method to add a name string to the People collection
    // code wouldn't need to know that we assign and i32 ID to names internally

    // creating type synonyms with type aliases
}
