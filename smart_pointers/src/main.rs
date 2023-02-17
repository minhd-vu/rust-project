// We've already dealt with smart pointers in the past. String and Vec<T> are two
// types of smart pointers. Smart pointers implement the Drop and Deref traits.
// Some additional values of smart pointers include Box<T> which is used for
// declaring items on the heap, Rc<T> which counts references and allows for
// multiple ownership, and RefCell<T> which enforces borrowing at runtime rather
// than compile time.
fn main() {
    println!("Hello, world!");
}
