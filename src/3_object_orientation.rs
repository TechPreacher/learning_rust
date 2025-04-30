// Object
struct Accumulator {
    sum: i32
}

impl Accumulator {
    // Constructor
    fn new(init: i32) -> Accumulator { // Associated function
        Accumulator { sum: init }
    }

    fn new2(sum: i32) -> Self { // Associated function
        Self { sum }
    }

    fn get(self) -> i32 // Method
    {
        self.sum
    }
}

fn main() {
    // Using the Accumulator struct
    let acc = Accumulator { sum: 0 }; // Create an instance of the Accumulator struct
    println!("acc = {:?}", acc.sum); // Print the instance of the Accumulator struct

    // Using the new constructor
    let acc = Accumulator::new(1); // Create an instance of the Accumulator struct using the new constructor
    println!("acc = {:?}", acc.sum); // Print the instance of the Accumulator struct

    // Using the new2 constructor
    let acc = Accumulator::new2(2); // Create an instance of the Accumulator struct using the new2 constructor
    println!("acc = {:?}", acc.sum); // Print the instance of the Accumulator struct

    // Using the get method
    let acc = Accumulator::new(3);
    println!("acc = {:?}", acc.get()); // Print the instance of the Accumulator struct using the get method
}

// NOTES:
// - When defining a field in an object (struct), each field must have a type.
// - Impl are not required to be in the same file as the struct.