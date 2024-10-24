mod cli;
mod data;
mod playback;
mod recording;

use cli::{parse_args, CliAction};

fn main() {
    // Match the command-line arguments to determine the action
    match parse_args() {
        // Handle the 'record' action
        CliAction::Record { session_name } => {
            let session_file = format!("{}.json", session_name);
            // Attempt to start the recording session and handle errors
            if let Err(err) = recording::record_session(&session_file) {
                eprintln!("An error occurred during recording: {:?}", err);
            }
        }
        // Handle the 'play' action
        CliAction::Play { session_name } => {
            let session_file = format!("{}.json", session_name);
            // Attempt to play the session and handle errors
            if let Err(err) = playback::play_session(&session_file) {
                eprintln!("An error occurred during playback: {:?}", err);
            }
        }
        // Handle the 'list' action
        CliAction::ListSessions => {
            // Attempt to list saved sessions and handle errors
            if let Err(err) = list_sessions() {
                eprintln!("An error occurred while listing sessions: {:?}", err);
            }
        }
        // Display the help message if no valid action is provided
        CliAction::ShowHelp => {
            print_help();
        }
    }
}

// Function to list saved sessions
fn list_sessions() -> std::io::Result<()> {
    use std::fs;

    let paths = fs::read_dir(".")?;
    println!("Saved sessions:");
    // Iterate through the directory and print the session files with .json extension
    for path in paths {
        let path = path?;
        if let Some(extension) = path.path().extension() {
            if extension == "json" {
                if let Some(filename) = path.path().file_stem() {
                    println!("- {}", filename.to_string_lossy());
                }
            }
        }
    }
    Ok(())
}

// Function to print the help message
fn print_help() {
    use clap::Command;

    let mut app = Command::new("monkey-typer")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("CLI tool to support live coding style performances")
        // Define the 'record' subcommand
        .subcommand(
            Command::new("record")
                .about("Starts a new recording session")
                .arg(
                    clap::Arg::new("SESSION")
                        .help("Specify the session name")
                        .required(true)
                        .index(1),
                ),
        )
        // Define the 'play' subcommand
        .subcommand(
            Command::new("play").about("Plays a recorded session").arg(
                clap::Arg::new("SESSION")
                    .help("Specify the session name to play")
                    .required(true)
                    .index(1),
            ),
        )
        // Define the 'list' subcommand
        .subcommand(Command::new("list").about("Displays a list of saved sessions"));

    app.print_help().unwrap();
    println!();
}
