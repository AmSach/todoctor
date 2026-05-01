use crate::exec::exec;
use std::error::Error;

pub async fn get_modified_files(
    previous_commit: &str,
    current_commit: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let output = exec(&[
        "git",
        "diff",
        "--name-status",
        previous_commit,
        current_commit,
    ])
    .await?;

    Ok(output
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                parts[1].to_string()
            } else {
                String::new()
            }
        })
        .filter(|s| !s.is_empty())
        .collect())
}
