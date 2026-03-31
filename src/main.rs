mod cli;
mod client;
mod commands;
mod models;

use clap::Parser;
use cli::{Cli, Command, CustomfieldsCommand, ProjectsCommand};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Version => {
            commands::version::execute();
            Ok(())
        }
        Command::Customfields { action } => match action {
            CustomfieldsCommand::List { project, skip, top } => {
                commands::customfields::execute_list(
                    cli.base_url.as_deref(),
                    project.as_deref(),
                    skip,
                    top,
                )
            }
            CustomfieldsCommand::Describe { project, name } => {
                commands::customfields::execute_describe(
                    cli.base_url.as_deref(),
                    project.as_deref(),
                    &name,
                )
            }
            CustomfieldsCommand::Create {
                project,
                name,
                field_type,
                auto_attach,
            } => commands::customfields::execute_create(
                cli.base_url.as_deref(),
                project.as_deref(),
                &name,
                &field_type,
                auto_attach,
            ),
            CustomfieldsCommand::CreateValue {
                project,
                field_name,
                value,
            } => commands::customfields::execute_create_value(
                cli.base_url.as_deref(),
                project.as_deref(),
                &field_name,
                &value,
            ),
        },
        Command::Projects { action } => match action {
            ProjectsCommand::List { skip, top } => {
                commands::projects::execute_list(cli.base_url.as_deref(), skip, top)
            }
            ProjectsCommand::Describe { name } => {
                commands::projects::execute_describe(cli.base_url.as_deref(), &name)
            }
        },
    };

    if let Err(e) = result {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
