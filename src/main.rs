mod fibonacci;
mod extract_numbers;
mod post_comment_to_github;

use crate::{fibonacci::fibonacci, extract_numbers::extract_numbers, post_comment_to_github::post_comment};
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Retrieve action inputs: enable_fib, max_threshold, etc.
    let enable_fib = args.get(1).unwrap_or(&"true".to_string()).to_lowercase() == "true";
    let max_threshold: u8 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);

    // Get pull request number from environment with fallback
    let pr_number: u64 = match env::var("PR_NUMBER") {
        Ok(val) => val.parse::<u64>().unwrap_or_else(|_| {
            eprintln!("Invalid PR_NUMBER format, using default value.");
            0 // Default value if PR_NUMBER is not set or invalid
        }),
        Err(_) => {
            eprintln!("PR_NUMBER not set, using default value.");
            0 // Default value if PR_NUMBER is not set
        }
    };

    println!("FibBot application is running...");
    println!("Fibonacci Calculation Enabled: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Fetch pull request content using GitHub API
    let pr_content = get_pr_content(pr_number).await;
    if pr_content.is_empty() {
        println!("No content found in this pull request.");
        return;
    }

    println!("Pull Request Content: {}", pr_content);

    // Extract numbers from PR content
    let numbers = extract_numbers(&pr_content);
    println!("Extracted Numbers: {:?}", numbers);

    // Calculate Fibonacci for numbers
    let mut response = String::from("#### Fibonacci output of each number in the pull request:\n");
    for &num in &numbers {
        if u64::from(num) <= max_threshold as u64 {  // Ensure it does not exceed max_threshold
            let fib = fibonacci(u64::from(num)); // Convert num to u64
            response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
        } else {
            response.push_str(&format!("- Fibonacci({}) = Skipped (exceeds max threshold)\n", num));
        }
    }

    println!("Fibonacci Results:\n{}", response);

    // Post result back to GitHub
    if let Err(e) = post_comment(&response).await {
        eprintln!("Error posting comment: {}", e);
    }
}

async fn get_pr_content(_pr_number: u64) -> String {
    // Fetch PR content from GitHub (this should include content like PR descriptions or comments)
    // Placeholder code, actual implementation needs to use GitHub API to fetch PR content.
    format!("This is a test PR with numbers: 3, 5, 8, 13, 21")
}
