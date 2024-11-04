use std::error::Error;
use std::io::{stdout, Write};

use crossterm::event::{read, Event as CEvent, KeyCode as CKeyCode, KeyEvent as CKeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};

use crate::data::{Key, KeyEvent, KeyEventReader};

/// Function to play a recorded session from a file
pub fn play_session(session_file: &str) -> Result<(), Box<dyn Error>> {
    // Open the file for playback
    let mut reader = KeyEventReader::open(session_file)?;

    println!("Playback started. Press any key to replay events. Press 'q' to quit.");

    // Enable raw mode to capture key presses without Enter
    enable_raw_mode()?;

    // Main loop to read and process key events
    loop {
        // Wait for user key press
        let event = read()?;
        if let CEvent::Key(CKeyEvent { code, .. }) = event {
            // Check if the user wants to quit
            if code == CKeyCode::Char('q') {
                break;
            }

            // Read the next recorded key event
            match reader.read_event() {
                Some(Ok(recorded_event)) => {
                    // Output the key event to the terminal
                    write_key_event(&recorded_event)?;
                }
                Some(Err(err)) => {
                    eprintln!("Error occurred while reading key event: {:?}", err);
                    break;
                }
                None => {
                    println!("\nPlayback finished.");
                    break;
                }
            }
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode()?;

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
                // Handle backspace
                execute!(stdout, cursor::MoveLeft(1))?;
                write!(stdout, " ")?;
                execute!(stdout, cursor::MoveLeft(1))?;
            }
            _ => {}
        }
    }
    // Ensure the output is flushed to the terminal
    stdout.flush()?;
    Ok(())
}
