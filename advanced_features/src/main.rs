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
