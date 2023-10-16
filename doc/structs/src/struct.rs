struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // compiler will complain that it needs lifetime specifiers
    let mut user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };

    user1.email = "anotheremail@example.com";

    let user2 = User {
        email: "another@example.com",
        ..user1
    }; //n this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // tuple struct instances are similar to
    // tuples in that you can destructure them
    // into their individual pieces, and you
    // can use a . followed by the index to
    // access an individual value.

    let subject = AlwaysEqual;
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
