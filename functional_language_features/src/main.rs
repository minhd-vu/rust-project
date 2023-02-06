use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // We get the user preferences in the form of the option. unwrap_or_else
    // which will return the value in Some if it is there or it will return the
    // value in the closure when it is None. In this context, this means that
    // it will return the most_stocked shirt color if the user does not have a
    // preference.
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // The closure takes no parameters and the body calls self.most_stocked().
        // Note that the closure captures the immutable reference to self,
        // functions differ in this way as this would have to be passed in.
        // Closures don't require type annotations like in function headers.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // All of the following are equal.
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;

    // Closures are not allowed to be called with two different types when the
    // types are inferred. Closures can be called like functions.
    add_one_v3(10);
    // This would cause an error.
    // add_one_v3(10.1);
}

// List is still accessible outside of the closure and after calling the closure
// because it is an immutable reference.
fn borrow_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

// Here the closure now captures a mutable reference. We can only borrow mutably
// once here or else there will be an error.
fn mutable_borrow_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // Adding this will cause an error.
    // list.push(10);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// If we want to closure to take ownership of the values then you can use the
// move keyword. This is useful if you want to move the data into another thread.
fn ownership_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// Closures will implement a one to three of these Fn traits:
// FnOnce: the closure can be called at least once, all closures must implement
// at least this trait.
// FnMut: applies to closures that don't move captured values by may mutate
// captured values and can be called more than once.
// Fn: applies to closures that don't mutate or move captured values and can be
// called more than once without mutating the environment.
