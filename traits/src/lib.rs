use std::fmt::Display;

// Traits are similar to interfaces but not exactly.
pub trait Summary {
    // This is a behavior that needs to be implemented by types. Here a default
    // implementation is defined. Default implementations can also call other
    // methods, even if they do not have a default implementation.
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This is how a trait is implemented on a type.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// You can't implement external traits on external types. This is called coherence
// or the orphan rule. Prevents two people from implementing the same trait for
// the same type.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// This function accepts any item type that implements the Summary trait. You can
// specify more than on trait.
pub fn notify(item: &(impl Summary + PartialOrd)) {
    println!("Breaking news! {}", item.summarize());
}

// The above is syntax sugar for the below, called a trait bound. You should use
// a trait bound if multiple types should be the same type.
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// The where clause can make trait bounds a little clearer.
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Clone + PartialOrd,
{
    return 10;
}

// This is how you would return a trait. You can only use this if returning a
// single type.
fn returns_summarizalbe() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Here, pair has two implementations. new() is for all T whereas cmp_display()
// is only for T that implement the Display and PartialOrd. This is conditional
// implementations.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
