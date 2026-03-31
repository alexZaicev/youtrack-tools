use anyhow::{Context, Result};

use crate::client::YouTrackClient;

fn build_client(base_url: Option<&str>) -> Result<YouTrackClient> {
    let api_key =
        std::env::var("YOUTRACK_API_KEY").context("YOUTRACK_API_KEY env var is not set")?;

    let url = base_url.context("YouTrack base URL is not set. Use --base-url or set the YOUTRACK_BASE_URL env var")?;
    YouTrackClient::new(url, &api_key)
}

pub fn execute_list(
    base_url: Option<&str>,
    project: Option<&str>,
    skip: Option<u32>,
    top: Option<u32>,
) -> Result<()> {
    let client = build_client(base_url)?;

    match project {
        Some(proj) => print_project_custom_fields(&client, proj, skip, top),
        None => print_global_custom_fields(&client, skip, top),
    }
}

fn print_global_custom_fields(
    client: &YouTrackClient,
    skip: Option<u32>,
    top: Option<u32>,
) -> Result<()> {
    let fields = client.list_custom_fields(skip, top)?;

    if fields.is_empty() {
        println!("No custom fields found.");
        return Ok(());
    }

    // Compute column widths
    let name_width = fields
        .iter()
        .map(|f| f.name.as_deref().unwrap_or("-").len())
        .max()
        .unwrap_or(4)
        .max(4);

    let type_width = fields
        .iter()
        .map(|f| {
            f.field_type
                .as_ref()
                .and_then(|ft| ft.id.as_deref())
                .unwrap_or("-")
                .len()
        })
        .max()
        .unwrap_or(4)
        .max(4);

    println!("{:<name_width$}  {:<type_width$}  ID", "NAME", "TYPE",);

    for cf in &fields {
        let name = cf.name.as_deref().unwrap_or("-");
        let ft = cf
            .field_type
            .as_ref()
            .and_then(|ft| ft.id.as_deref())
            .unwrap_or("-");
        let id = cf.id.as_deref().unwrap_or("-");
        println!("{:<name_width$}  {:<type_width$}  {}", name, ft, id);
    }

    Ok(())
}

fn print_project_custom_fields(
    client: &YouTrackClient,
    project: &str,
    skip: Option<u32>,
    top: Option<u32>,
) -> Result<()> {
    let fields = client.list_project_custom_fields(project, skip, top)?;

    if fields.is_empty() {
        println!("No custom fields found for project '{project}'.");
        return Ok(());
    }

    let name_width = fields
        .iter()
        .map(|f| {
            f.field
                .as_ref()
                .and_then(|fld| fld.name.as_deref())
                .unwrap_or("-")
                .len()
        })
        .max()
        .unwrap_or(4)
        .max(4);

    let type_width = fields
        .iter()
        .map(|f| {
            f.field
                .as_ref()
                .and_then(|fld| fld.field_type.as_ref())
                .and_then(|ft| ft.id.as_deref())
                .unwrap_or("-")
                .len()
        })
        .max()
        .unwrap_or(4)
        .max(4);

    println!(
        "{:<name_width$}  {:<type_width$}  PUBLIC  ID",
        "NAME", "TYPE",
    );

    for pcf in &fields {
        let name = pcf
            .field
            .as_ref()
            .and_then(|fld| fld.name.as_deref())
            .unwrap_or("-");
        let ft = pcf
            .field
            .as_ref()
            .and_then(|fld| fld.field_type.as_ref())
            .and_then(|ft| ft.id.as_deref())
            .unwrap_or("-");
        let public = pcf
            .is_public
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".to_string());
        let id = pcf.id.as_deref().unwrap_or("-");
        println!(
            "{:<name_width$}  {:<type_width$}  {:<6}  {}",
            name, ft, public, id
        );
    }

    Ok(())
}

pub fn execute_describe(base_url: Option<&str>, project: Option<&str>, name: &str) -> Result<()> {
    let client = build_client(base_url)?;

    match project {
        Some(proj) => match client.get_project_custom_field_by_name(proj, name)? {
            Some(pcf) => println!("{pcf}"),
            None => anyhow::bail!("custom field '{}' not found in project '{}'", name, proj),
        },
        None => match client.get_custom_field_by_name(name)? {
            Some(cf) => println!("{cf}"),
            None => anyhow::bail!("custom field '{}' not found", name),
        },
    }

    Ok(())
}

pub fn execute_create(
    base_url: Option<&str>,
    project: Option<&str>,
    name: &str,
    field_type: &str,
    auto_attach: Option<bool>,
) -> Result<()> {
    let client = build_client(base_url)?;

    // Always create the global field first
    let cf = client.create_custom_field(name, field_type, auto_attach)?;
    println!("Created custom field:");
    println!("{cf}");

    // If a project was specified, attach the newly created field to it
    if let Some(proj) = project {
        let cf_id = cf
            .id
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("created field has no ID"))?;
        let pcf = client.attach_custom_field_to_project(proj, cf_id)?;
        println!();
        println!("Attached to project '{proj}':");
        println!("{pcf}");
    }

    Ok(())
}

pub fn execute_create_value(
    base_url: Option<&str>,
    project: Option<&str>,
    field_name: &str,
    value: &str,
) -> Result<()> {
    let client = build_client(base_url)?;

    let element = match project {
        Some(proj) => client.add_value_to_project_custom_field(proj, field_name, value)?,
        None => client.add_value_to_custom_field(field_name, value)?,
    };

    let name = element.name.as_deref().unwrap_or("-");
    let id = element.id.as_deref().unwrap_or("-");
    println!("Created value '{name}' (id: {id})");

    Ok(())
}
