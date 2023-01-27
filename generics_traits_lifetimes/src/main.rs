fn main() {
    println!("Hello, world!");
}

// This is a generic function that calculates the largest value in a list. If the
// PartialOrd trait is omitted, this function will not compile. This ensures that
// we can only use values that are ordered.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// You can also use generics in structs. Here x and y have the same type, so if a
// Point is constructed where x and y are different, it will not compile. Similar
// to struct, enums can you generics as well. Recall the Option<T> and
// Result<T, E> enums.
struct Point<T> {
    x: T,
    y: T,
}

// This is how you would use generics in a method. We have to declare T in the
// impl so that we can use it in the Point<T>.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // Here notice that methods don't always need to have the same type as the
    // struct. A different type T2 is used here to illustrate this.
    fn mixup<T2>(self, other: Point<T2>) -> Point<T2> {
        other
    }
}

// Here only points types of f32 will have the distance_from_origin method. Other
// methods of type T will not have this method defined.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
