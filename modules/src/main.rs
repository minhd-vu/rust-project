mod my_module {
    pub struct MyStruct {

    }

    pub fn do_something() {
        println!("something");
    }
}

// Here, my_module has to be in the same scope and do_something must be public in order to access.
// Similar principles apply to structs and enums as well. The key difference being the structs
// individual fields can have the pub keyword. Generally, this would not be idiomatic because for
// functions you want to import the module, not the full path. For structs...
use my_module::do_something;
// This would be idiomatic to specify the full path for structs, enums, etc. You can also use the
// as keyword to rename something upon import. You can also re-export something by adding the pub
// keyword before use.
pub use my_module::MyStruct as Struct;

// You can use the self keyword in an import which in this example means use my_module.
// use my_module::{self, MyStruct};
// You can also bring in all public items definied into scope by using the glob opeartor or the *.

fn main() {
    println!("Hello, world!");
    do_something();
}
