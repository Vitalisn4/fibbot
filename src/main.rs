use std::env;

fn main() {

     println!("Hello, World");

    // Retrieve arguments passed to the GitHub Action (for testing purposes)
    let args: Vec<String> = env::args().collect();

    // Retrieve enable_fib flag (true/false) and max_threshold (default 100)
    let enable_fib = args
        .get(1)
        .unwrap_or(&"true".to_string())
        .to_lowercase() == "true";
    let max_threshold: u8 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);

    // Print the parsed values for debugging and confirmation
    println!("FibBot is running...");
    println!("Fibonacci Calculation Enabled: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Additional logic will be added later to handle PR number and calculate Fibonacci
}
