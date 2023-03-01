fn main() {
    println!("Hello, world of macros! ❗❓❔❕");
    // declarative macros w/ macro_rules!
    // procedural macros
    // 1. Custom #[derive]: specify code added with the derive attribute used on structs and enums
    // 2. Attribute-like: define custom attributes usable on any item
    // 3. Function-like: look like function calls but operate on the tokens specified as their argument
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec
// }
