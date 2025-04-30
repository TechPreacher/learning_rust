// Closures

fn main() {
    // Closure with explicit return type
    println!("\nClosure with explicit return type");
    let is_even = | val: i32 | -> bool {
        if (val % 2) == 0 {
            return true;
        }
        return false;
    };

    for n in 3..10 {
        println!("{} is even: {}", n, is_even(n));
    }

    // Simpler Closure with inferred return type
    println!("\nSimpler Closure with inferred return type");
    let is_even_simpler = | val: i32 | val % 2 == 0; // No need for return statement, the last expression is returned automatically.

    for n in 3..10 {
        println!("{} is even: {}", n, is_even_simpler(n))
    }

    // Closure with inferred types
    println!("\nClosure with inferred types");
    let mut sum: i32 = 0;
    let add = | n1, n2 | n1 + n2; // A closure that takes two parameters and returns their sum. Types are inferred.

    for n in 3..10 {
        sum = add(sum, n);
    }
    println!("The sum of the range is {}", sum);

    // Closure can make use of variables from the surrounding scope
    println!("\nClosure can make use of variables from the surrounding scope");
    let add = | n1, n2 | n1 + n2;
    let sum_range = | start, end | {
        let mut sum: i32 = 0;
        for n in start..end {
            sum = add(sum, n);
        }
        sum
    };
    println!("Sum of range 3..10 is {}", sum_range(3, 10));

    // Closure with captured variables from the surrounding scope (🧠🧨)
    println!("\nClosure with captured variables from the surrounding scope");
    let add = | n1, n2 | n1 + n2;
    let (from, to) = (3, 10);
    let sum_range = || {
        let mut sum = 0;
        for n in from..to {
            sum = add(sum, n);
        }
        sum
    };
    println!("Sum of range 3..10 is {}", sum_range());

    // Iterators
    println!("\nIterators");
    let mut sum: i32 = 0;
    let add = | n1, n2 | n1 + n2;
    let numbers = (3..10)
        .inspect(| n | println!("n = {}", n)); // Inspect each number

    for n in numbers {
        sum = add(sum, n);
    }
    println!("The sum of the range is {}", sum);

    // Iterator chaining
    println!("\nIterator chaining");
    let mut sum: i32 = 0;
    let add = | n1, n2 | n1 + n2;
    let numbers = (3..10)
        .inspect(| n | println!("Before filter, n = {}", n))
        .filter(|n| n % 2 == 0) // Filter even numbers
        .inspect(| n | println!("After filter, n = {}", n)); // Inspect filtered numbers

    for n in numbers {
        sum = add(sum, n);
    }
    println!("The sum of the filtered range is {}", sum);
    
    // Using built-in fold function
    println!("\nUsing built-in fold function");
    let sum = (3..10)
        .filter(|n| n % 2 == 0) // Filter even numbers
        .fold(0, | tally, n | tally + n); // Fold the iterator to get the sum
    
    println!("sum = {}", sum); // Print the sum
    
    // Using the sum function (generics)
    println!("\nUsing the sum function (generics)");
    let sum = (3..10)
        .filter(|n| n % 2 == 0) // Filter even numbers
        .sum::<i32>(); // Sum the iterator
    println!("The sum of the filtered range is {}", sum); // Print the sum
    
    // Using the sum function (explicit)
    println!("\nUsing the sum function (explicit)");
    let sum: i32 = (3..10)
        .filter(|n| n % 2 == 0) // Filter even numbers
        .sum(); // Sum the iterator
    println!("The sum of the filtered range is {}", sum); // Print the sum
}

// NOTES:
// - When using generics and Rust can't automatically infer the type, you can use the `::<Type>` syntax to specify the type.