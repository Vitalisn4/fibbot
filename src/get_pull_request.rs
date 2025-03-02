use crate::extract_numbers::extract_numbers;
use octocrab::{Octocrab, Result as OctoResult};
use std::env;

pub async fn get_pr_content(pr_number: u64) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo_name = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    
    let repo_parts: Vec<&str> = repo_name.split('/').collect();
    if repo_parts.len() != 2 {
        return Err("Invalid GITHUB_REPOSITORY format".into());
    }
    
    let owner = repo_parts[0];
    let repo = repo_parts[1];
    
    println!("Fetching PR content for {}/{} PR #{}", owner, repo, pr_number);
    
    let octocrab = match Octocrab::builder()
        .personal_token(token)
        .build() {
            Ok(client) => client,
            Err(e) => return Err(format!("Failed to build Octocrab client: {}", e).into())
        };
    
    // Get PR details
    let pr = octocrab
        .pulls(owner, repo)
        .get(pr_number)
        .await
        .map_err(|e| format!("Failed to get PR: {}", e))?;
    
    // Get PR description
    let description = pr.body.unwrap_or_default();
    
    // Get PR files
    let files = octocrab
        .pulls(owner, repo)
        .list_files(pr_number)
        .await
        .map_err(|e| format!("Failed to list PR files: {}", e))?;
    
    let mut all_content = String::new();
    all_content.push_str(&description);
    
    // Add patch content from files
    for file in files.items {
        if let Some(patch) = file.patch {
            all_content.push_str(&patch);
        }
    }
    
    println!("PR Content snippet: {:.100}...", all_content);
    let numbers = extract_numbers(&all_content);
    println!("Extracted numbers: {:?}", numbers);
    
    Ok(numbers)
}