//! Script to validate commands documentation against actual CLI implementation
//!
//! This script compares the documented commands in docs/commands.md against
//! the actual command definitions in src/types.rs to ensure they match.

use std::collections::HashMap;
use std::fs;

/// Represents a CLI command with its options
#[derive(Debug, Default)]
struct Command {
    name: String,
    description: String,
    options: Vec<CommandOption>,
    subcommands: Vec<String>,
}

/// Represents a command option/parameter
#[derive(Debug, Default)]
struct CommandOption {
    name: String,
    short: Option<String>,
    long: Option<String>,
    description: String,
    required: bool,
    default: Option<String>,
}

/// Parse commands from the actual implementation (src/types.rs)
fn parse_implementation_commands() -> HashMap<String, Command> {
    let mut commands = HashMap::new();

    // This would parse the actual Commands enum from src/types.rs
    // For now, we'll just add some example commands to show the structure

    // Add example commands that we know exist
    let mut add_cmd = Command {
        name: "add".to_string(),
        description: "Add new items to Todozi".to_string(),
        ..Default::default()
    };

    add_cmd.options.push(CommandOption {
        name: "action".to_string(),
        description: "Task description".to_string(),
        required: true,
        ..Default::default()
    });

    add_cmd.options.push(CommandOption {
        name: "time".to_string(),
        short: Some("t".to_string()),
        long: Some("time".to_string()),
        description: "Time estimate".to_string(),
        required: false,
        ..Default::default()
    });

    commands.insert("add".to_string(), add_cmd);

    commands
}

/// Parse commands from the documentation (docs/commands.md)
fn parse_documented_commands() -> HashMap<String, Command> {
    let mut commands = HashMap::new();

    // This would parse the markdown documentation
    // For now, we'll just add example entries

    let add_cmd = Command {
        name: "add".to_string(),
        description: "Add new items to Todozi".to_string(),
        ..Default::default()
    };

    commands.insert("add".to_string(), add_cmd);

    commands
}

/// Compare implementation commands with documented commands
fn validate_commands() -> Result<(), Box<dyn std::error::Error>> {
    let impl_commands = parse_implementation_commands();
    let doc_commands = parse_documented_commands();

    println!(
        "Validating {} implemented commands against {} documented commands...",
        impl_commands.len(),
        doc_commands.len()
    );

    // Check for missing documentation
    for (name, impl_cmd) in &impl_commands {
        if !doc_commands.contains_key(name) {
            println!("⚠️  Missing documentation for command: {}", name);
        }
    }

    // Check for outdated documentation
    for (name, doc_cmd) in &doc_commands {
        if !impl_commands.contains_key(name) {
            println!(
                "❌ Documented command not found in implementation: {}",
                name
            );
        }
    }

    // Check option consistency for matching commands
    for (name, impl_cmd) in &impl_commands {
        if let Some(doc_cmd) = doc_commands.get(name) {
            // Compare options
            for impl_option in &impl_cmd.options {
                let found = doc_cmd
                    .options
                    .iter()
                    .any(|doc_opt| doc_opt.name == impl_option.name);

                if !found {
                    println!(
                        "⚠️  Missing documentation for option '{}' in command '{}'",
                        impl_option.name, name
                    );
                }
            }
        }
    }

    println!("Validation complete!");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    validate_commands()
}
