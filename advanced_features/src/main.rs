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

// This is a type alias or synonym. Using this method, we don't get the type
// checking benefits from the newtype pattern like in Meters and Millimeters.
// This use case is good for reducing repetition of writing long types. Result<T>
// is one such alias that means Result<T, Error>.
type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_and_returns_long_type(f: Thunk) -> Thunk {
    Box::new(|| println!("hi"))
}

// str is a dynamically sized type, which means the size of it must be evaluated
// at runtime. &str on the other hand stores, two things: the address of the str
// and its length.
// let s1: str = "Hello there!";
// let s2: str = "How's it going?";

// By default, all generic functions work on types that have a known size at
// compile time, to override this:
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// Use the fn type to pass a function pointer. Because function pointers
// implement all the traits of a closure, they can be passed in where you would
// usually expect a closure.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_pointers() {
    let answer = do_twice(add_one, 5);

    let list_of_numbers = vec![1, 2, 3];
    // Here you can either pass a closure or a function pointer to map.
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    // You can also use enum initializer functions as function pointers.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// To return a closure do the following. Notice that we have to use the Box trait
// because we don't know how large the closure will be.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
