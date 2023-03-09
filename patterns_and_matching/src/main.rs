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

fn refutability_example() {
    // This is an irrefutable statement because x will match anything.
    let x = 5;
    let option: Option<&str> = None;

    // This is a refutable statement because if the value of color is not some,
    // then the pattern will fail to match.
    // let Some(some) = option;
    // To use the refutable pattern we must handle the case in which it is none.
    // We can do this with an if statement.
    if let Some(some) = option {
        // Therefore, match arms must be refutable patterns, and something like
        // if let x = 5 {} would be useless.
    }
}

fn matching_literals() {
    let x = 1;

    match x {
        // You can match multiple patterns with the | syntax.
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        // To match an inclusive range.
        4..=10 => println!("four to ten"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        // This first arm doesn't match because the value of x is 5.
        Some(50) => println!("Got 50"),
        // The next arm matches all values and stores the value in y. This y
        // shadows the other y in this scope.
        Some(y) => println!("Matched, y = {y}"),
        // If x is None, then this arm would match.
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

struct Point {
    x: i32,
    y: i32,
}

fn destructure_structs() {
    let p = Point { x: 0, y: 7 };

    // Create two variables a and b from Point p.
    let Point { x: a, y: b } = p;
    // You can also use the direct struct field names.
    let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Matching also works with destructuring.
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
