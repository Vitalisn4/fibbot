mod fibonacci; 
mod extract_numbers; 

use crate::{fibonacci::fibonacci, extract_numbers::extract_numbers};

use std::env;

fn main() {

 // Retrieve arguments if passed in the GitHub Action run (for testing purposes)
    let args: Vec<String> = env::args().collect();

    println!("Hello, World");

    // Simulate Pull Request Content for testing
    let pr_content = "The numbers to calculate Fibonacci for are: 3, 5, 8, 13, and 21";

    // Extract numbers from PR content
    let numbers = extract_numbers(pr_content);
    println!("Extracted Numbers: {:?}", numbers);

    // Calculate Fibonacci for extracted numbers
    for &num in &numbers {
        let fib = fibonacci(num);
        println!("Fibonacci({}) = {}", num, fib);
    }
}
