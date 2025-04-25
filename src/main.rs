fn add(a: i32, b: i32) -> i32 {
    a + b // The function returns the sum of a and b
}

fn main() {
    // The classic "Hello, World!" program
    println!("Hello, world!");

    // A simple addition example
    let a: i32 = 10; // Statically, strongly typed
    let b: i32 = 20;
    let sum: i32 = a + b;
    println!("The sum of {} and {} is {}", a, b, sum); // ! is used to call a macro, not a function
    
    // Mutability example
    let a: i32 = 10;
    let b: i32 = 20;
    let mut sum: i32 = a; // Immutable by default - use mut to make it mutable
    sum += b; // Using the += operator to add b to sum
    println!("The sum of {} and {} is {}", a, b, sum);

    // Using a function to perform addition
    let a: i32 = 10;
    let b: i32 = 20;
    let sum: i32 = add(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);
    
    // Using an array and loop
    let values: [i32; 5] = [1, 2, 3, 4, 5];
    let mut sum: i32 = 0;
    
    for n in values {
        sum = add(sum, n); // Adding each element to sum
    }
    println!("The sum of the array is {}", sum);
    
    // Array slicing
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
    let mut values: Vec<i32> = vec![8, 30]; // Creating a vector with initial values
    let mut sum: i32 = 0;
    
    values.push(1); // Adding elements to the vector
    values.push(3);
    
    for n in values {
        sum = add(sum, n);
    }
    println!("The sum of the vector is {}", sum);
}
