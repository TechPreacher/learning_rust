fn add(a: i32, b: i32) -> i32 {
    a + b // The function returns the sum of a and b
}

fn main() {
    // The classic "Hello, World!" program
    println!("Hello, world!");

    // A simple addition example
    println!("\nA simple addition example");
    let a: i32 = 10; // Statically, strongly typed
    let b: i32 = 20;
    let sum: i32 = a + b;
    println!("The sum of {} and {} is {}", a, b, sum); // ! is used to call a macro, not a function

    // Mutability example
    println!("\nMutability example");
    let a: i32 = 10;
    let b: i32 = 20;
    let mut sum: i32 = a; // Immutable by default - use mut to make it mutable
    sum += b; // Using the += operator to add b to sum
    println!("The sum of {} and {} is {}", a, b, sum);

    // Using a function to perform addition
    println!("\nUsing a function to perform addition");
    let a: i32 = 10;
    let b: i32 = 20;
    let sum: i32 = add(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);

    // Using an array and loop
    println!("\nUsing an array and loop");
    let values: [i32; 5] = [1, 2, 3, 4, 5];
    let mut sum: i32 = 0;

    for n in values {
        sum = add(sum, n); // Adding each element to sum
    }
    println!("The sum of the array is {}", sum);

    // Simplification
    println!("\nSimplification");
    let mut sum: i32 = 0;

    for n in 3..10 { // Using a range to iterate from 3 to 10
        sum = add(sum, n); // Adding each element to sum
    }
    println!("The sum of the range is {}", sum);

    // Array slicing
    println!("\nArray slicing");
    let values: [i32; 5] = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for n in &values[0..3] { // Slicing the array to get the first three elements. Returns a reference
        sum = add(sum, *n); // Adding each element to sum. * to de-reference the value
    }
    for n in &values[3..5] { // Slicing the array to get the last two elements. Returns a reference
        sum = add(sum, *n); // Adding each element to sum. * to de-reference the value
    }
    // Note: Rust arrays are NOT growable. The size of an array must be known at compile time.
    println!("The sum of the sliced array is {}", sum);

    // Vectors sample
    println!("\nVectors sample");
    let mut values: Vec<i32> = vec![8, 30]; // Creating a vector with initial values
    let mut sum: i32 = 0;

    values.push(1); // Adding elements to the vector
    values.push(3);

    for n in values {
        sum = add(sum, n);
    }
    println!("The sum of the vector is {}", sum);

    // Lazy iterators
    println!("\nLazy iterators");
    let sum = 0;
    let _add = |n1:i32, n2:i32| n1 + n2; // A closure that takes two parameters and returns their sum. Types are inferred.
    let _numbers = (3..10)
        .inspect(|n| println!("n = {}", n)); // Inspect each number.
    println!("The sum of the range is {}", sum);
}

// NOTES:
// Zero-cost abstraction: Safety checks are done at compile time, not at runtime.
// Standard library of commonly useful types available.
// Functional-Flavored Object-Oriented Language: Rust is a multi-paradigm language.
// Rust supports closures: functions that can capture their enclosing environment.
// Adding a _ to a variable like _myvar signals that the variable is not used (on purpose).
