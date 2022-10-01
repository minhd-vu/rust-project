use std::collections::HashMap;

fn main() {
    // Create a new empty vector. Here rust doesn't know what type we want the empty vector to have
    // so we must give it a type annotation.
    let v: Vec<i32> = Vec::new();
    // You can also use the vec macro to initialize a vector with some values.
    let mut v = vec![1, 2, 3];
    // Add elements to a vector by using push method.
    v.push(5);
    // To get an element at an index you can either use the get method or the []. Using the former
    // will return an option while using the latter will panic if the index is not found.

    // To iterate over the values in a Vector, use the in keyword to iterate over elements.
    // If there was no &v then it would be a move instead of a borrow so be cafeful!
    for i in &v {
        println!("{}", i);
    }
    // Vectors can only hold one type but if multiple types are needed you can use an enum.

    // Strings are implemented as a collection of bytes. Rust only has one string type in the core
    // language which is the string slice str which is the same as string literals. The String type
    // is provided in Rust's standard library.

    // Creating a new empty string.
    let mut s = String::new();
    // The to_string method will return a String. You can also use String::from to do the same
    // thing here.
    let mut s = "hello world".to_string();
    // You can grow a string by using push_str which wil append the string. push_str does not take
    // ownership.
    s.push_str("hello");
    // You can push a single charater by using the push method.
    s.push('l');
    // Using the + operator on strings without the & will move the string.
    let s1 = String::from("Hello");
    let s2 = String::from("Hi");
    // Here we require an owned string on the left, so s1 will be moved whilets2 is not. Here, the
    // compiler actually coerces the type of s2 into a &str from a &String.
    let s3 = s1 + &s2;
    // For more complicated string concatenation, use the format macro.
    let s = format!("{}{}{}", s3, s2, s2);
    // Rust strings do not support indexing because indexing would return the first byte, but Rust
    // strings use UTF-8 in which characters are not exactly one byte. You can take a slice of a
    // string, but the boundaries must be of char boundaries.
    // You can iterate over a string, but you need to be explicit about it, use the chars or bytes
    // method to specify what you want.
    for c in s.chars() {
        println!("{}", c);
    }

    // Hash maps store mappings of keys to values. To create a HashMap use the new method.
    let mut map = HashMap::new();
    // To insert values into the hash map use the insert method. Inserting will move values with
    // the Copy trait. Inserting will replace the value if it alerady exists.
    map.insert(String::from("Hello"), String::from("World"));
    // Get a value from the hash map by using the get method.
    let value = map.get("Hello");
    // To iterate over a map use the in keyword.
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
    // The entry method returns an Entry enum which represents whether the value may or may not
    // exist.
    let entry = map.entry(String::from("Hello"));
}

fn get_median(nums: &[i32]) -> i32 {
    let mut vec = nums.to_vec();
    vec.sort();
    vec[vec.len() / 2]
}

fn get_mode(nums: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &n in nums {
        *map.entry(n).or_insert(0) += 1;
    }
    map.iter()
        .max_by_key(|&(_, v)| v)
        .map(|(k, _)| *k)
        .expect("failed to compute mode")
}
