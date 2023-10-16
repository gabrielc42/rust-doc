use oop::Post;

pub fn blog() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // assert_eq!("", post.content()); // should give error
    assert_eq!("I ate a salad for lunch today", post.content());

    post.reject();

    assert_eq!("", post.content());
}
