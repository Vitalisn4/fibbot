use crate::extract_numbers::extract_numbers;
use octocrab::Octocrab;

pub async fn get_pr(pr_number: u64) -> String {
    // Initialize octocrab instance
    let octocrab = Octocrab::builder().build().expect("Failed to create Octocrab instance");

    // Replace these with your actual repo details
    let repo_owner = "Vitalisn4"; // Your GitHub username
    let repo_name = "fibbot"; // Your GitHub repository name

    // Fetch PR content (this could be the body or description of the PR)
    let pr = octocrab
        .pulls(repo_owner, repo_name)
        .get(pr_number)
        .await
        .expect("Failed to fetch pull request");

    // Simulate PR content for testing (replace with real data)
    let pr_content = pr.body.unwrap_or_else(|| String::from(""));

    println!("Fetched PR content: {}", pr_content);

    pr_content
}
