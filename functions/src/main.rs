fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

// Functions always need typed parameters. Function definitions are statements.
// You cannot do something like x = y = 6 in Rust.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

    // The new block scope is an expression, and it evaluates to something.
    // Expressions do not include semicolons, statements do.
    let y = {
        let x = 3;
        // The return value of the function is synonymous with the value of the final
        // expression in the block of the body of a function.
        x + 1
    };

    println!("The value of y is: {y}");

    // if is an expression so we can use it in a let statement.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    let mut counter = 0;
    // Loop labels begin with a single quote.
    let result = 'loop_label: loop {
        counter += 1;
        if counter == 10 {
            // Return the value after the break keyword to be assigned to result.
            break 'loop_label counter * 2;
        }
    };
    println!("The result is {result}");

    let a = [10, 20, 30, 40, 50];
    // For loops can use the in keyword.
    for element in a {
        println!("the value is: {element}");
    }
}
