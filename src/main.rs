mod prelude;
mod errors;
mod logging;
mod config;

mod format { pub mod tks_json; }
mod engine { pub mod playhead; pub mod engine; }
mod io { pub mod keyboard; }
mod output { pub mod injector; pub mod os_inject; }

use crate::prelude::*;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "monkey-typer", about = "Live coding air-typing performer", version)]
struct Cli {
    #[arg(global=true, short='f', long="file")]
    file: Option<PathBuf>,

    #[arg(global=true, long="global", default_value_t=false)]
    global: bool,

    #[arg(global=true, long="trigger", default_value="right")]
    trigger: String,

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
        Commands::Livecli => run_live_cli(cli.file, cli.global, cli.trigger)?,
    }
    Ok(())
}

fn run_live_cli(file: Option<PathBuf>, use_global: bool, trigger: String) -> Result<()> {
    use engine::engine::Engine;
    use output::os_inject::OsInjector;

    let path = file.expect("specify .tks.json");
    let bytes = std::fs::read(path)?;
    let seq = format::tks_json::Sequence::from_bytes(&bytes)?;

    let mut engine = Engine::new(seq.actions, seq.markers);
    let mut injector = OsInjector::new();
    let mut lat_samples: Vec<f64> = Vec::new();

    let trig_key = io::keyboard::TriggerKey::parse(&trigger);

    if use_global {
        println!("Live CLI (GLOBAL): {:?}=Step, Ctrl+P=Pause, Esc=Panic", trig_key);
        let trig = io::keyboard::GlobalTrigger::new(trig_key);
        loop {
            if let Some(ev) = trig.try_recv() {
                use io::keyboard::TriggerEvent::*;
                match ev {
                    Step => { let t0=std::time::Instant::now(); engine.step(&mut injector)?; lat_samples.push(t0.elapsed().as_secs_f64()*1000.0);
                              if engine.head.is_eof(){ println!("\n[Done]"); break; } }
                    PauseToggle => { engine.toggle_pause(); println!("\r[{}]   ", if engine.head.paused { "Paused" } else { "Running" }); }
                    Panic => { engine.panic_stop(); println!("\n[PANIC]"); break; }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    } else {
        use crossterm::terminal;
        terminal::enable_raw_mode()?;
        println!("Live CLI (LOCAL): {:?}=Step, Ctrl+P=Pause, Esc=Panic", trig_key);
        loop {
            if let Some(ev) = io::keyboard::poll_trigger_local(1, trig_key) {
                use io::keyboard::TriggerEvent::*;
                match ev {
                    Step => { let t0=std::time::Instant::now(); engine.step(&mut injector)?; lat_samples.push(t0.elapsed().as_secs_f64()*1000.0);
                              if engine.head.is_eof(){ println!("\n[Done]"); break; } }
                    PauseToggle => { engine.toggle_pause(); println!("\r[{}]   ", if engine.head.paused { "Paused" } else { "Running" }); }
                    Panic => { engine.panic_stop(); println!("\n[PANIC]"); break; }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(0));
        }
        crossterm::terminal::disable_raw_mode()?;
    }

    if !lat_samples.is_empty() {
        let avg = lat_samples.iter().copied().sum::<f64>() / (lat_samples.len() as f64);
        println!("Latency: {:.2}", avg);
    }
    Ok(())
}