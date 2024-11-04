use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::data::{KeyEvent, KeyEventWriter};
use rdev::{listen, ListenError};

/// Records a session and saves key events to a file.
pub fn record_session(session_file: &str) -> Result<(), ListenError> {
    // Open a file for recording
    let writer = Arc::new(Mutex::new(KeyEventWriter::open(session_file).unwrap()));

    // Flag indicating whether to continue recording
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

    // Run the event listener on a separate thread
    let writer_clone = writer.clone();
    let running_clone = running.clone();
    let listener_thread = std::thread::spawn(move || {
        if let Err(err) = listen(move |event| {
            if !running_clone.load(Ordering::SeqCst) {
                // Exit the listener
                return;
            }

            // Process the event
            if let Some(key_event) = KeyEvent::from_rdev_event(&event) {
                let mut writer = writer_clone.lock().unwrap();
                if let Err(err) = writer.write_event(&key_event) {
                    eprintln!("Error occurred while writing key event: {:?}", err);
                }
            }
        }) {
            eprintln!("Error occurred while listening for events: {:?}", err);
        }
    });

    // Wait until the `running` flag becomes `false`
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Recording stopped.");

    // Wait for the listener thread to terminate
    listener_thread.join().unwrap();

    Ok(())
}
