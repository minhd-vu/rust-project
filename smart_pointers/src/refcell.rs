use std::{cell::RefCell, rc::Rc};

fn refcell_example() {
    // Where as Box<T> is borrowing rules are enforced at compile time,
    // RefCell<T>'s are enforced at runtime. If the borrowing rules with RefCell
    // are broken, the program will panic and exit. RefCell can only be used for
    // single threaded scenarios. RefCell allows for mutable borrows at runtime
    // and you can mutate the value inside RefCell even if RefCell is immutable.
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     struct MockMessenger {
//         sent_messages: Vec<String>,
//     }
//
//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: vec![],
//             }
//         }
//     }
//
//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             // There will be an error here because &self is immutable, and if
//             // we try to change it to &mut self, it will conflict with the
//             // Messenger trait definition.
//             self.sent_messages.push(String::from(message));
//         }
//     }
//
//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
//
//         limit_tracker.set_value(80);
//
//         assert_eq!(mock_messenger.sent_messages.len(), 1);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut() will return a mutable reference (RefMut<T>). This
            // allows &self to still be an immutable reference while we modify
            // the value inside of the RefCell.
            //
            // RefCell will keep track of how many immutable borrows or mutable
            // borrows happen at runtime. This is just the same borrow rules at
            // compile time but happening at runtime. It will allow us to have
            // many immutable borrows or one mutable borrow; if these conditions
            // are violated there will be a panic.
            self.sent_messages.borrow_mut().push(String::from(message));
        }

        // For example using borrow_mut() twice below will cause a runtime panic.
        // fn send(&self, message: &str) {
        //     let mut one_borrow = self.sent_messages.borrow_mut();
        //     let mut two_borrow = self.sent_messages.borrow_mut();
        //
        //     one_borrow.push(String::from(message));
        //     two_borrow.push(String::from(message));
        // }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // Calling borrow() will get an immutable reference (Ref<T>).
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn rc_refcell_example() {
    // Because Rc only stores immutable we use a RefCell so that we can have
    // mutable access to it later by calling borrow_mut().
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Mutably borrow and dereference. There are runtime checks for borrowing
    // rules so we don't run into data races.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
