use std::env;

fn main() {
    // Retrieve arguments if passed in the GitHub Action run (for testing purposes)
    let args: Vec<String> = env::args().collect();

    // Print a basic message to confirm the action is running
    println!("Hello, world!");

    // Optionally, print the received arguments for further debugging
    if !args.is_empty() {
        println!("Arguments passed: {:?}", args);
    }
}

