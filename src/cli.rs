use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "youtrack-tools", about = "A collection of YouTrack utilities")]
pub struct Cli {
    /// YouTrack API base URL (overrides YOUTRACK_BASE_URL env var)
    #[arg(long, global = true, env = "YOUTRACK_BASE_URL")]
    pub base_url: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Print version, OS/arch, and Rust compiler information
    Version,

    /// Manage YouTrack custom field resources
    Customfields {
        #[command(subcommand)]
        action: CustomfieldsCommand,
    },

    /// Manage YouTrack projects
    Projects {
        #[command(subcommand)]
        action: ProjectsCommand,
    },
}

#[derive(Subcommand)]
pub enum CustomfieldsCommand {
    /// List all custom fields
    List {
        /// Project short name to scope fields to (e.g. "PROJ")
        #[arg(short, long)]
        project: Option<String>,

        /// Number of items to skip (pagination offset)
        #[arg(long)]
        skip: Option<u32>,

        /// Maximum number of items to return (pagination limit)
        #[arg(long)]
        top: Option<u32>,
    },

    /// Describe a custom field by name
    Describe {
        /// Project short name to scope fields to (e.g. "PROJ")
        #[arg(short, long)]
        project: Option<String>,

        /// Name of the custom field to look up
        name: String,
    },

    /// Create a new custom field (globally or attach to a project)
    Create {
        /// Project short name — when provided, creates the field globally then attaches it to the project
        #[arg(short, long)]
        project: Option<String>,

        /// Name of the custom field
        name: String,

        /// Field type id (e.g. "enum[1]", "state[1]", "string", "integer", "date", "period")
        #[arg(short = 't', long = "type")]
        field_type: String,

        /// Auto-attach this field to all projects
        #[arg(long)]
        auto_attach: Option<bool>,
    },

    /// Create a new value in a custom field's bundle
    CreateValue {
        /// Project short name to scope fields to (e.g. "PROJ")
        #[arg(short, long)]
        project: Option<String>,

        /// Name of the custom field to add the value to
        #[arg(short = 'f', long = "field")]
        field_name: String,

        /// Name of the new value to create
        value: String,
    },
}

#[derive(Subcommand)]
pub enum ProjectsCommand {
    /// List all projects
    List {
        /// Number of items to skip (pagination offset)
        #[arg(long)]
        skip: Option<u32>,

        /// Maximum number of items to return (pagination limit)
        #[arg(long)]
        top: Option<u32>,
    },

    /// Describe a project by short name
    Describe {
        /// Short name of the project to look up (e.g. "PROJ")
        name: String,
    },
}
