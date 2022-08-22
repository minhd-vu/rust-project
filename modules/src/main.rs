mod my_module {
    pub fn do_something() {
        println!("something");
    }
}

// Here, my_module has to be in the same scope and do_something must be public in order to access.
// Similar principles apply to structs and enums as well. The key difference being the structs
// individual fields can have the pub keyword.
use my_module::do_something;

fn main() {
    println!("Hello, world!");
    do_something();
}
