use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::data::{KeyEvent, KeyEventWriter};
use rdev::{listen, ListenError};

/// Function to record a session and save key events to a file
pub fn record_session(session_file: &str) -> Result<(), ListenError> {
    // Open the file for recording
    let writer = Arc::new(Mutex::new(KeyEventWriter::open(session_file).unwrap()));

    // Flag to indicate whether recording should continue
    let running = Arc::new(AtomicBool::new(true));

    // Set up Ctrl+C signal handling
    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })
        .unwrap();
    }

    println!("Recording started. Press Ctrl+C to stop.");

    // Event handler
    let writer_clone = writer.clone();
    let running_clone = running.clone();

    // Capture key events
    listen(move |event| {
        if !running_clone.load(Ordering::SeqCst) {
            // End recording
            std::process::exit(0);
        }

        // Convert the event to a KeyEvent and write it to the file
        if let Some(key_event) = KeyEvent::from_rdev_event(&event) {
            let mut writer = writer_clone.lock().unwrap();
            if let Err(err) = writer.write_event(&key_event) {
                eprintln!("Error occurred while writing key event: {:?}", err);
            }
        }
    })
}
