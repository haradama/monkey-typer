use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use crate::data::{open_event_reader, Key, KeyEvent};
use std::io::{stdout, Write};

/// Function to play a recorded session from a file
pub fn play_session(session_file: &str) -> Result<(), Box<dyn Error>> {
    // Open the file for playback
    let mut reader = open_event_reader(session_file)?;

    println!("Playback started.");

    // Main loop to read and process key events
    while let Some(event_result) = reader.read_event() {
        match event_result {
            Ok(event) => {
                // Output the key event to the terminal
                write_key_event(&event)?;
                // Add a sleep to control playback speed (adjust as necessary)
                sleep(Duration::from_millis(20));
            }
            Err(err) => {
                eprintln!("Error occurred while reading key event: {:?}", err);
                break;
            }
        }
    }

    println!("\nPlayback finished.");

    Ok(())
}

/// Output the key event to the terminal
fn write_key_event(event: &KeyEvent) -> std::io::Result<()> {
    let mut stdout = stdout();
    if let Some(text) = &event.text {
        // If the event contains text, write it directly
        write!(stdout, "{}", text)?;
    } else {
        // Handle different key types
        match event.key {
            Key::Return => {
                write!(stdout, "\n")?;
            }
            Key::Space => {
                write!(stdout, " ")?;
            }
            Key::Tab => {
                write!(stdout, "\t")?;
            }
            Key::Backspace => {
                write!(stdout, "\x08 \x08")?; // Handle backspace
            }
            _ => {}
        }
    }
    // Ensure the output is flushed to the terminal
    stdout.flush()?;
    Ok(())
}
