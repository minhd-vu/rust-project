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
}
