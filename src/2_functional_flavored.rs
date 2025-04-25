// Closures

fn main() {
    // Closure with explicit return type
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
    let is_even_simpler = | val: i32 | val % 2 == 0; // No need for return statement, the last expression is returned automatically.

    for n in 3..10 {
        println!("{} is even: {}", n, is_even_simpler(n))
    }

    // Closure with inferred types
    let mut sum: i32 = 0;
    let add = | n1, n2 | n1 + n2; // A closure that takes two parameters and returns their sum. Types are inferred.

    for n in 3..10 {
        sum = add(sum, n);
    }
    println!("The sum of the range is {}", sum);

    // Closure can make use of variables from the surrounding scope
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
    let mut sum: i32 = 0;
    let add = | n1, n2 | n1 + n2;
    let numbers = (3..10)
        .inspect(| n | println!("n = {}", n)); // Inspect each number

    for n in numbers {
        sum = add(sum, n);
    }
    println!("The sum of the range is {}", sum);
}
