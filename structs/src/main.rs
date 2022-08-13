fn main() {
    println!("Hello, world!");

    //  This derive(Debug) allows us to use println!("{:?}") or :#? (pretty-print) on the struct.
    #[derive(Debug)]
    struct Car {
        wheels: i32,
        doors: i32,
        model: String,
        manufacturer: String,
    }

    // Here we are defining a drive method for the Car struct. All functions defined in the impl 
    // block are called associated functions. There can multiple impl blocks for one type.
    impl Car {
        // &self is actually short for self: &Self. This self is immutable, but it can also be done mutably.
        fn drive(&self) {
            println!("I am driving a car with {} wheels!", self.wheels);
        }
        
        // Often the method name is the same as the field, this is a getter for the wheels field.
        // Rust has automatic referencing and dereferencing so using the . operator will automatically
        // add the necessary &, &mut, or *.
        fn wheels(&self) -> i32 {
            self.wheels
        }
        
        // This is an associated function but not a method because it does not take self as
        // a parameter. Here Self is an alias for the type that appears after the impl keyword.
        fn prius() -> Self {
            Self {
                wheels: 4,
                doors: 4,
                model: "Prius",
                manufacturer: "Toyota",
            }
        }
    }

    let wheels = 4;
    let car = Car {
        wheels,
        doors: 4,
        model: String::from("Prius"),
        manufacturer: String::from("Toyota"),
    };

    // Struct update syntax. Note that this moves the values of car.model and
    // car.manufacturer so car is no longer valid.
    let car2 = Car {
        wheels: 5,
        ..car
    };

    // println!("{}", car.model);

    // Tuple structs have struct name but no field names.
    struct Color(i32, i32, i32);

    // This is a unit struct it evaluates to () which is a unit. This is neat 
    // because you can put a behavior on this type.
    struct UnitStruct;

    // dbg returns ownership of expression in side and prints out to stderr.
    let a = dbg!(30);
}
