fn main() {
    println!("Hello, world of raw identifiers!");

    // raw identifiers are syntax that allow keywords where they wouldn't normally be allowed
    // prefix a keyword with #r

    // this fn compiles into an error
    // fn match(needle: &str, haystack: &str) -> bool {
    //     haystack.contains(needle)
    // }

    // instead
    fn r#match(needle: &str, haystack: &str) -> bool {
        haystack.contains(needle)
    }

    assert!(r#match("foo", "foobar"));
}
