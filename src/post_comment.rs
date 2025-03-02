use reqwest::Client;
use serde_json::json;
use std::env;

pub async fn post_comment(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u64>()
        .expect("Invalid PR_NUMBER");
    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );
    println!("Posting comment to {}", url);
    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "FibBot-GitHub-Action")
        .json(&json!({
            "body": content
        }))
        .send()
        .await?;
    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Failed to post comment. Status: {}, Error: {}", 
                  status, error_text);
        return Err(format!("API request failed: {}", error_text).into());
    }
    Ok(())
}