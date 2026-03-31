use anyhow::{Context, Result};

use crate::client::YouTrackClient;

fn build_client(base_url: Option<&str>) -> Result<YouTrackClient> {
    let api_key =
        std::env::var("YOUTRACK_API_KEY").context("YOUTRACK_API_KEY env var is not set")?;

    let url = base_url.context("YouTrack base URL is not set. Use --base-url or set the YOUTRACK_BASE_URL env var")?;
    YouTrackClient::new(url, &api_key)
}

pub fn execute_list(base_url: Option<&str>, skip: Option<u32>, top: Option<u32>) -> Result<()> {
    let client = build_client(base_url)?;
    let projects = client.list_projects(skip, top)?;

    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    // Compute column widths
    let name_width = projects
        .iter()
        .map(|p| p.name.as_deref().unwrap_or("-").len())
        .max()
        .unwrap_or(4)
        .max(4);

    let short_width = projects
        .iter()
        .map(|p| p.short_name.as_deref().unwrap_or("-").len())
        .max()
        .unwrap_or(5)
        .max(5);

    let leader_width = projects
        .iter()
        .map(|p| {
            p.leader
                .as_ref()
                .and_then(|l| l.login.as_deref())
                .unwrap_or("-")
                .len()
        })
        .max()
        .unwrap_or(6)
        .max(6);

    println!(
        "{:<name_width$}  {:<short_width$}  {:<leader_width$}  ARCHIVED  ID",
        "NAME", "SHORT", "LEADER",
    );

    for p in &projects {
        let name = p.name.as_deref().unwrap_or("-");
        let short = p.short_name.as_deref().unwrap_or("-");
        let leader = p
            .leader
            .as_ref()
            .and_then(|l| l.login.as_deref())
            .unwrap_or("-");
        let archived = p
            .archived
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".to_string());
        let id = p.id.as_deref().unwrap_or("-");
        println!(
            "{:<name_width$}  {:<short_width$}  {:<leader_width$}  {:<8}  {}",
            name, short, leader, archived, id
        );
    }

    Ok(())
}

pub fn execute_describe(base_url: Option<&str>, name: &str) -> Result<()> {
    let client = build_client(base_url)?;

    match client.get_project_by_name(name)? {
        Some(project) => println!("{project}"),
        None => anyhow::bail!("project with short name '{}' not found", name),
    }

    Ok(())
}
