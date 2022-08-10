fn main() {
    // Variables are immutable by default.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants are always immutable and can be declared in the global scope.
    // Constants can only be set to constant expression, i.e. it cannot be
    // evaluated at runtime. Constant naming is uppercase snake.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Scope matters when shadowing. Shadowing allows for types to change whereas
    // using mutable variables does not.
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Default floating point number is f64.
    let y = 2.0;

    // Chars are denoted with single quotes and are four bytes in size (unicode).
    let heart_eyed_cat = '😻';

    // Tuples are fixed length and variable types.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // To destructure a tuple.
    let (x, y, z) = tup;
    // To access a single value of the tuple.
    let six_point_four = tup.1;

    // Arrays have fixed length and the same type.
    // let name: [type; size] = ...;
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5] == [3, 3, 3, 3, 3];
}
