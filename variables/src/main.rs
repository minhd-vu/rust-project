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
}
