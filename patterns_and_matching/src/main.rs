fn main() {
    if_let_example();
    while_let_example();
    for_example();
    print_coordinates(&(0, 0))
}

fn if_let_example() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // The if let statement is similar to a match statement, but you only match
    // one arm. This is different than a match because you can intersperse other
    // conditions.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    }
    // Here, a shadowed age variable is introduced.
    else if let Ok(age) = age {
        // The shadowed variable doesn't enter scope until the brackets.
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let_example() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // The while let allows the while loop to run as the pattern continues to
    // match.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_example() {
    let v = vec!['a', 'b', 'c'];
    // The keyword that follows the for is a pattern. Here, the pattern is for x
    // in y. Using the enumerate iterator here allows us to access the indexes
    // by destructuring the tuple.
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_example() {
    // We use a pattern with let to destructure a tuple.
    let (x, y, z) = (1, 2, 3);
}

// Here, we pattern match the parameters in the function by destructuring a tuple.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
