mod config;
mod errors;
mod logging;
mod prelude;

mod format {
    pub mod tks_json;
}
mod engine {
    pub mod engine;
    pub mod playhead;
}
mod io {
    pub mod keyboard;
}
mod output {
    pub mod injector;
    pub mod os_inject;
}

use crate::{output::injector::OutputInjector, prelude::*};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "monkey-typer",
    about = "Live coding air-typing performer",
    version
)]
struct Cli {
    #[arg(global = true, short = 'f', long = "file")]
    file: Option<PathBuf>,

    #[arg(global = true, long = "global", default_value_t = false)]
    global: bool,

    #[arg(
        global = true,
        long = "step-key",
        alias = "trigger",
        default_value = "right"
    )]
    step_key: String,

    #[arg(global = true, long = "pause-key", default_value = "ctrl+p")]
    pause_key: String,

    #[arg(global = true, long = "start-key", default_value = "ctrl+s")]
    start_key: String,

    #[arg(global = true, long = "exit-key", default_value = "ctrl+q")]
    exit_key: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Livecli,
}

fn main() -> Result<()> {
    logging::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Livecli => run_live_cli(cli)?,
    }
    Ok(())
}

fn run_live_cli(cli: Cli) -> Result<()> {
    use engine::engine::Engine;
    use output::os_inject::OsInjector;

    let path = cli.file.expect("specify .tks.json");
    let bytes = std::fs::read(path)?;
    let seq = format::tks_json::Sequence::from_bytes(&bytes)?;

    let mut engine = Engine::new(seq.actions, seq.markers);
    let mut injector = OsInjector::new();
    let mut lat_samples: Vec<f64> = Vec::new();
    let keys = io::keyboard::Hotkeys::from_strings(
        &cli.step_key,
        &cli.pause_key,
        &cli.start_key,
        &cli.exit_key,
    );

    let mut started = false;

    if cli.global {
        println!(
            "Live CLI (GLOBAL)\n  Start={:?} / Step={:?} + [a-z] / Pause=Ctrl+P / Exit=Ctrl+Q",
            keys.start, keys.step
        );
        let trig = io::keyboard::GlobalTrigger::new(keys);

        loop {
            if let Some(ev) = trig.try_recv() {
                use io::keyboard::TriggerEvent::*;
                match ev {
                    Start => {
                        if !started {
                            started = true;
                            println!("[Started]");
                        }
                    }
                    Step { from_alpha } => {
                        if !started {
                            continue;
                        }
                        if from_alpha {
                            injector.backspace()?;
                            std::thread::sleep(std::time::Duration::from_millis(1));
                        }
                        let t0 = std::time::Instant::now();
                        engine.step(&mut injector)?;
                        lat_samples.push(t0.elapsed().as_secs_f64() * 1000.0);
                        if engine.head.is_eof() {
                            println!("\n[Done]");
                            break;
                        }
                    }
                    PauseToggle => {
                        engine.toggle_pause();
                        println!(
                            "\r[{}]   ",
                            if engine.head.paused {
                                "Paused"
                            } else {
                                "Running"
                            }
                        );
                    }
                    Exit => {
                        println!("\n[Exit]");
                        break;
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    } else {
        use crossterm::terminal;
        terminal::enable_raw_mode()?;
        println!(
            "Live CLI (LOCAL)\n  Start={:?} / Step={:?} + [a-z] / Pause=Ctrl+P / Exit=Ctrl+Q",
            keys.start, keys.step
        );

        loop {
            if let Some(ev) = io::keyboard::poll_trigger_local(1, &keys) {
                use io::keyboard::TriggerEvent::*;
                match ev {
                    Start => {
                        if !started {
                            started = true;
                            println!("[Started]");
                        }
                    }
                    Step { from_alpha } => {
                        if !started {
                            continue;
                        }
                        if from_alpha {
                            injector.backspace()?;
                            std::thread::sleep(std::time::Duration::from_millis(1));
                        }
                        let t0 = std::time::Instant::now();
                        engine.step(&mut injector)?;
                        lat_samples.push(t0.elapsed().as_secs_f64() * 1000.0);
                        if engine.head.is_eof() {
                            println!("\n[Done]");
                            break;
                        }
                    }
                    PauseToggle => {
                        engine.toggle_pause();
                        println!(
                            "\r[{}]   ",
                            if engine.head.paused {
                                "Paused"
                            } else {
                                "Running"
                            }
                        );
                    }
                    Exit => {
                        println!("\n[Exit]");
                        break;
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(0));
        }
        crossterm::terminal::disable_raw_mode()?;
    }

    if !lat_samples.is_empty() {
        let avg = lat_samples.iter().copied().sum::<f64>() / (lat_samples.len() as f64);
        println!("Latency(avg per step): {:.2} ms", avg);
    }
    Ok(())
}
