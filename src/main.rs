use clap::Parser;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod config;
mod cpu_stress;
mod disk_ops;
mod utils;

use config::Config;
use cpu_stress::CpuStressor;
use disk_ops::DiskTester;

#[derive(Parser)]
#[command(about = "Disk stress testing tool")]
struct Args {
    #[arg(long)]
    dir: PathBuf,
    
    #[arg(long, default_value = "4")]
    chunk_size_gb: u64,
    
    #[arg(long, default_value = "90")]
    cpu_load: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nShutting down...");
    })?;

    println!("Starting stress test...");
    println!("Press Ctrl+C to stop");

    let config = Config::new(args.dir, args.chunk_size_gb, args.cpu_load)?;
    let cpu_stressor = CpuStressor::new(config.cpu_load);
    let disk_tester = DiskTester::new(&config)?;

    tokio::select! {
        _ = cpu_stressor.run(running.clone()) => {},
        result = disk_tester.run(running.clone()) => {
            if let Err(e) = result {
                eprintln!("Disk test error: {}", e);
            }
        }
    }

    Ok(())
}