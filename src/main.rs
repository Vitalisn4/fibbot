mod extract_numbers;
mod fibonacci;
mod get_pull_request;
mod post_comment;

use crate::{
    fibonacci::fibonacci,
    extract_numbers::extract_numbers,
    get_pull_request::get_pr_content,
    post_comment::post_comment,
};

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    let enable_fib = args
        .get(1)
        .unwrap_or(&"true".to_string())
        .to_lowercase() == "true";
        
    let max_threshold: u32 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);
    
    println!("FibBot GitHub Action is running...");
    println!("Fibonacci calculation enabled: {}", enable_fib);
    println!("Max threshold: {}", max_threshold);
    
    // Check if running in GitHub Actions
    let github_actions = env::var("GITHUB_ACTIONS").unwrap_or_default() == "true";
    
    if github_actions {
        // Get PR number from environment
        let pr_number = match env::var("PR_NUMBER") {
            Ok(num) => match num.parse::<u64>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid PR_NUMBER value");
                    return Ok(());
                }
            },
            Err(_) => {
                // Try to extract from GITHUB_REF
                if let Ok(github_ref) = env::var("GITHUB_REF") {
                    if github_ref.starts_with("refs/pull/") && github_ref.contains("/merge") {
                        let parts: Vec<&str> = github_ref.split('/').collect();
                        if parts.len() >= 3 {
                            match parts[2].parse::<u64>() {
                                Ok(n) => n,
                                Err(_) => 0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        };
        
        if pr_number == 0 {
            println!("Not running on a pull request or PR_NUMBER not set");
            return Ok(());
        }
        
        println!("Processing PR #{}", pr_number);
        
        // Get PR content and extract numbers
        let numbers = match get_pr_content(pr_number).await {
            Ok(nums) => nums,
            Err(e) => {
                eprintln!("Error fetching PR content: {}", e);
                return Ok(());
            }
        };
        
        if numbers.is_empty() {
            println!("No numbers found in PR content");
            let comment = "No numbers found in this pull request to calculate Fibonacci values.";
            post_comment(comment).await?;
            return Ok(());
        }
        
        // Filter numbers based on threshold
        let filtered_numbers: Vec<u32> = numbers
            .into_iter()
            .filter(|&n| n <= max_threshold)
            .collect();
            
        if filtered_numbers.is_empty() {
            let comment = format!(
                "All numbers found in the PR exceed the maximum threshold of {}.", 
                max_threshold
            );
            post_comment(&comment).await?;
            return Ok(());
        }
        
        // Calculate Fibonacci numbers
        let mut comment = String::from("## FibBot Results\n\nFibonacci numbers for values found in this PR:\n\n");
        
        for &num in &filtered_numbers {
            if enable_fib {
                let fib = fibonacci(num);
                comment.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
            } else {
                comment.push_str(&format!("- Found number: {}\n", num));
            }
        }
        
        // Post comment to PR
        post_comment(&comment).await?;
    } else {
        // Test mode - use sample PR content
        println!("Running in test mode with sample PR content");
        let sample_pr = "This is a sample PR with numbers: 3, 5, 8, 13, 21";
        let numbers = extract_numbers(sample_pr);
        
        println!("Extracted numbers: {:?}", numbers);
        
        let mut results = String::from("## FibBot Test Results\n\n");
        for &num in &numbers {
            if enable_fib && num <= max_threshold {
                let fib = fibonacci(num);
                results.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
            }
        }
        
        println!("{}", results);
    }
    
    Ok(())
}