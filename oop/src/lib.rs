// Why not an enum? You may have to use a match everywhere, which may be more
// repetitive than a trait object solution.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content
            .push_str(self.state.as_ref().unwrap().add_text(text));
    }

    pub fn content(&self) -> &str {
        // as_ref is used because we want the reference to value to the Option
        // rather than to own the value.
        self.state.as_ref().unwrap().content(self)
    }

    // This method does the same thing regardless of the state value. The
    // variability lies in the structs implementing State.
    pub fn request_review(&mut self) {
        // take() moves the state value out Post rather than borrowing.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    // The parameter of Box<Self> will take ownership of the Box passed to it.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // This default implementation means, no implementation is required for Draft
    // and PendingReview states.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject<'a>(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        text
    }
}

struct PendingReview {
    approvals: i32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approvals += 1;
        if self.approvals >= 2 {
            Box::new(Published {})
        } else {
            self
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// If another state were to be added, then the transition between certain states
// would have to be redone.
