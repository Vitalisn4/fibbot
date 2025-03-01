use reqwest::Client;
use std::env;

pub async fn post_comment(pr_content: &str) -> Result<(), reqwest::Error> {
    // Handle missing GITHUB_REPOSITORY environment variable
    let repo = match env::var("GITHUB_REPOSITORY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("GITHUB_REPOSITORY not set, skipping comment posting.");
            return Ok(()); // Skip posting comment if GITHUB_REPOSITORY is not set
        }
    };

    // Get pull request number from environment with fallback
    let pr_number: u32 = match env::var("PR_NUMBER") {
        Ok(val) => val.parse::<u32>().unwrap_or_else(|_| {
            eprintln!("Invalid PR_NUMBER format, using default value.");
            0 // Default value if PR_NUMBER is not set or invalid
        }),
        Err(_) => {
            eprintln!("PR_NUMBER not set, using default value.");
            0 // Default value if PR_NUMBER is not set
        }
    };

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .json(&serde_json::json!({ "body": pr_content }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Comment posted successfully.");
    } else {
        eprintln!("Failed to post comment: {:?}", response.text().await?);
    }

    Ok(())
}
