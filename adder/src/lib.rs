// A common pattern, even for binary-only crates,
// is to declare a "library" providing the majority of functionality,
// and then have main.rs just implement argument parsing and use that library.

// This allows for integration tests written against the library,
// which might otherwise be hard to do with only main.rs.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
