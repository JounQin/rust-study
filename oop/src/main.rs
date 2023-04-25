use oop::{blog2, Button, Draw, Screen};
use oop::blog::Post;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn _sceen() {
    let sceen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    sceen.run();
}

fn _blog() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch ");
    post.add_text("today");
    assert_eq!("", post.content());
    post.request_review();
    post.add_text("today");
    assert_eq!("", post.content());
    post.add_text("today");
    post.approve();
    post.add_text("today");
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn _blog2() {
    let mut post = blog2::Post::new();
    post.add_text("I ate a salad for lunch ");
    post.add_text("today");
    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn main() {
    _blog2();
}
