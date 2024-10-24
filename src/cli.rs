use clap::{Arg, Command};

// Function to parse command-line arguments and return a corresponding action
pub fn parse_args() -> CliAction {
    // Create a command-line parser using Clap
    let matches = Command::new("monkey-typer")
        .version("0.1")
        .about("CLI tool to support live coding style performance")
        // Define the "record" subcommand
        .subcommand(
            Command::new("record")
                .about("Starts a new recording session")
                .arg(
                    Arg::new("SESSION")
                        .help("Specify the session name")
                        .required(true)
                        .index(1),
                ),
        )
        // Define the "play" subcommand
        .subcommand(
            Command::new("play").about("Plays a recorded session").arg(
                Arg::new("SESSION")
                    .help("Specify the session name to play")
                    .required(true)
                    .index(1),
            ),
        )
        // Define the "list" subcommand
        .subcommand(Command::new("list").about("Displays a list of saved sessions"))
        // Get the matches from the command-line input
        .get_matches();

    // Check which subcommand was used and return the corresponding action
    if let Some(matches) = matches.subcommand_matches("record") {
        let session_name = matches.get_one::<String>("SESSION").unwrap();
        // Return the Record action with the specified session name
        CliAction::Record {
            session_name: session_name.clone(),
        }
    } else if let Some(matches) = matches.subcommand_matches("play") {
        let session_name = matches.get_one::<String>("SESSION").unwrap();
        // Return the Play action with the specified session name
        CliAction::Play {
            session_name: session_name.clone(),
        }
    } else if matches.subcommand_matches("list").is_some() {
        // Return the ListSessions action
        CliAction::ListSessions
    } else {
        // If no valid subcommand is provided, show the help message
        CliAction::ShowHelp
    }
}

// Enum to represent the various actions based on command-line input
pub enum CliAction {
    Record { session_name: String }, // Action to start a recording session
    Play { session_name: String },   // Action to play a recorded session
    ListSessions,                    // Action to list all saved sessions
    ShowHelp,                        // Action to display the help message
}
