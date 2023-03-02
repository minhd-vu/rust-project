use oop::Post;

fn main() {
    // Some consider objects something that packages both data and procedures to
    // operate on that data object oriented. Rust has this in structs and enums,
    // and impl statements allow for methods. The idea of encapsulation is also
    // associated with OOP, rust provides this by using the pub keyword. Non-pub
    // code is hidden from the end user. Rust does not have inheritance, but you
    // can have limited inheritance using traits and default trait methods. Rust
    // uses traits and generics to have bounded parametric polymorphism.

    let screen = Screen {
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
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();

    blog_example();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // This allows the vector to hold a Box<Button> and a Box<TextField> at the
    // same time. If using generics and trait bounds, the entire vector would be
    // one type. Similar to duck typing.
    pub components: Vec<Box<dyn Draw>>,
    // Here we use the dyn keyword for dynamic dispatch. This means the compiler
    // doesn't know what will implement the Draw trait at compile time (e.g.
    // SelectBox). This is less performant than static dispatch but provides more
    // flexibility.
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw the button
    }
}

// Assume that the SelectBox and methods are in a different library. The original
// program doesn't need to be aware of them in order to use it.
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw the select box
    }
}

fn blog_example() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
