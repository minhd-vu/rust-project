fn main() {
    println!("Hello, world!");
}

fn unsafe_rust() {
    let mut num = 5;

    // This is how to create raw pointers from references. You can create raw
    // pointers outside of unsafe blocks, but you can't dereference then outside
    // of unsafe blocks.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Create a raw pointer to an arbitrary location in memory.
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        // Only when dereferencing raw pointer do we need to be inside a unsafe
        // block.
        println!("r1: {}, r2: {}", *r1, *r2);

        // Unsafe functions must be called within an unsafe block.
        dangerous();
    }
}

// See the split_at_mut() function to see how you can use call a safe function
// that uses the unsafe inside its body.
unsafe fn dangerous() {}

// Sometimes rust has to interact with code written in another language.
extern "C" {
    fn abs(input: i32) -> i32;
}

// Static variables can be mutable and have a fixed memory address.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe traits also exist.
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

struct Counter {}

// The two traits are similar but hold important differences. Where the generic
// implementation can be implemented for many types, where the associated trait
// can only be implemented for one type.
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        return None;
    }
}

// The add trait sets the default generic trait parameter.
trait Add<T = Self> {
    type Output;

    fn add(self, rhs: T) -> Self::Output;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Here the default Self is being used as the output.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// Here we override the default because we want to return the millimeters
// instead of meters.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn methods_with_same_name() {
    let person = Human;
    // To call the methods with the same name, use the syntax below to specify
    // what trait it is from.
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    // This will use the impl Dog function.
    println!("A baby dog is called a {}", Dog::baby_name());
    // This will use the Animal trait's method.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

// Display is a super trait for OutlinePrint.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Wrapper(Vec<String>);

// Using the Newtype pattern to implement the Display trait for a vec, where it
// usually wouldn't be allowed to because vec is defined outside of the crate.
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
