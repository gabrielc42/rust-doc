pub fn string() {
    let mut s = String::new();

    let hello = String::from("السلام عليكم");
    println!("{}", hello);

    let hello = String::from("Dobrý den");
    println!("{}", hello);

    let hello = String::from("Hello");
    println!("{}", hello);

    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);

    let hello = String::from("नमस्ते");
    println!("{}", hello);

    let hello = String::from("こんにちは");
    println!("{}", hello);

    let hello = String::from("안녕하세요");
    println!("{}", hello);

    let hello = String::from("你好");
    println!("{}", hello);

    let hello = String::from("Olá");
    println!("{}", hello);

    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // remember:
    // str is read-only view, not own memory it points to
    // String is a growable, mutable string type
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //string cannot be indexed ...-> error
    // let s1 = String::from("hello");
    // let h = s1[0];

    //iterating over strings

    for c in "Зд".chars() {
        println!("{c}");
    }
    // valid unicode scalar values may be made up of more than 1 byte
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
