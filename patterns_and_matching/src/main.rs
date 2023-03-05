fn main() {
    if_let_example();
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
