mod fibonacci;
mod extract_numbers;
mod get_pull_request;
mod post_comment_to_github;

use crate::{fibonacci::fibonacci, extract_numbers::extract_numbers, get_pull_request::get_pr, post_comment_to_github::post_comment};
use std::env;

#[tokio::main]
async fn main() {
    // Retrieve arguments if passed in the GitHub Action run (for testing purposes)
    let args: Vec<String> = env::args().collect();

    let enable_fib = args.get(1).unwrap_or(&"true".to_string()).to_lowercase() == "true";
    let max_threshold: u8 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);

    println!("Hello, World");
    println!("Fibonacci Calculation Enabled: {}", enable_fib);
    println!("Max Threshold is: {}", max_threshold);

    // Get PR number from the environment
    let pr_number: u64 = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u64>()
        .expect("Invalid PR_NUMBER");

    // Fetch PR content and extract numbers
    let pr_content = get_pr(pr_number).await;
    println!("PR Content: {:?}", pr_content);

    let numbers = extract_numbers(&pr_content);
    println!("Extracted Numbers: {:?}", numbers);

    // Prepare the response string
    let mut response = String::from("#### Fibonacci output of each number in the pull request:\n");

    // Calculate Fibonacci and build response
    for &num in &numbers {
        let fib = fibonacci(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }

    // Post the comment back to GitHub
    if let Err(e) = post_comment(&response).await {
        eprintln!("Error posting comment: {}", e);
    }
}
