// Using Rayon for this solution, research rayon::join. Once you have read into rayon::join, in the
// fibonacci_join function of the provided source code, create a solution that utilizes rayon::join
// so that we can see a performance enhancement in our Fibonacci solution. If you get an error stating
// that your stack is overflowing, please lower the argument (47) that is being passed into the functions.

// If you unsure of what the Fibonacci sequence is, it is a series of numbers where the next number is
// found by adding up the two numbers before it. Example of Fibonacci(6)= 1, 1, 2, 3, 5, 8

use rayon::prelude::*;
use std::time::{Duration, Instant};

fn fib_recursive(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fibonacci_join(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    //Solution to the assignment goes here
    let (a, b) = rayon::join(|| fib_recursive(n - 1), || fib_recursive(n - 2));

    a + b
}

pub fn assignment() {
    let start = Instant::now();
    let x = fib_recursive(47);
    let duration = start.elapsed();
    println!(
        "Recursive fibonacci answer: {}, time taken: {:?}",
        x, duration
    );

    println!("Now run with Rayon's join.");

    let start = Instant::now();
    let x = fibonacci_join(47);
    let duration = start.elapsed();
    println!("Rayon fibonacci answer: {}, time taken: {:?}", x, duration);
}
