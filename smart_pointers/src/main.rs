use crate::List::{Cons, Nil};

// We've already dealt with smart pointers in the past. String and Vec<T> are two
// types of smart pointers. Smart pointers implement the Drop and Deref traits.
// Some additional values of smart pointers include Box<T> which is used for
// declaring items on the heap, Rc<T> which counts references and allows for
// multiple ownership, and RefCell<T> which enforces borrowing at runtime rather
// than compile time.
fn main() {
    box_example();
    deref_example();
}

// Use box when you 1) don't know the size at compile time, 2) large amount of
// data to transfer ownership and not copy, 3) when you care about a value
// implementing a trait rather than a specific type.
fn box_example() {
    // Here we store the i32 value of 5 on the heap rather than the default place
    // on the stack.
    let b = Box::new(5);
    println!("b = {}", b);
    // When b goes out of scope, both the box and the value it contains will be
    // deallocated.
    // Boxes are useful for recursive types, because we don't know the size of
    // the data at compile time or how deep the recursion will go.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn deref_example() {
    // Because Box<T> implements the deref trait, the deref operator can be used
    // on it similar to a reference. The difference here is y is pointing to a
    // copy of x lrather than the original one if we were to not use the Box<T>
    // type.
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // The dereference above is actually the same as:
    // *(y.deref());
    // This allows rust to work the same if it's a regular reference or a trait
    // that implements Deref.

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // If we try the below without implementing the deref trait, it won't compile
    // because the rust will not know how to dereference MyBox.
    assert_eq!(5, *y);
    // We can't do something like y.drop() because rust will still call drop at
    // when y goes out of scope, causing a double free. The way to drop something
    // early would be:
    drop(y)
}

enum List {
    // Without the box, this would be a type with infinite size, we use box to
    // indicate that the recursive List will be stored on the heap.
    Cons(i32, Box<List>),
    Nil,
}

// Here we define our own box struct.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // This declares an associated type.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // This will return the first value in the tuple struct.
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    // This will be called when MyBox goes out of scope.
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

// Deref coercion can convert &String to &str because String implements the
// Deref trait. Deref coercion only works on types that implement the Deref trait.

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercion() {
    hello("World!");

    // Because the Deref trait is implemented:
    // &MyBox<String> -> &String -> &str
    // This conversion is handled automatically and makes it clearer.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // You can also use the DerefMut trait for dereference mutable references.
}

use std::rc::Rc;
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_example() {
    // Rc can only be used to share references in single threaded scenarios. If
    // we were to use a Box<RcList> instead of a Rc<RcList> then a move error
    // would show up. This allows b and c to both point to a.
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    // a.clone() is different than Rc::clone because Rc::clone only increments
    // the reference counter. Once the variable goes out of scope, the reference
    // count will be decremented with the Drop trait. We can use Rc::strong_count
    // and Rc::weak_count to get the number of references. Rc allows a single
    // value to have multiple owners and only be cleaned up when no more owners
    // are left.
    let b = RcList::Cons(3, Rc::clone(&a));
    let c = RcList::Cons(4, Rc::clone(&a));
}
