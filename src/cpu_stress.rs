use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::task;

pub struct CpuStressor {
    target_load: u8,
}

impl CpuStressor {
    pub fn new(target_load: u8) -> Self {
        Self { target_load }
    }

    pub async fn run(&self, running: Arc<AtomicBool>) {
        let num_cores = num_cpus::get();
        let mut handles = vec![];

        for _ in 0..num_cores {
            let running = running.clone();
            let handle = task::spawn_blocking(move || {
                while running.load(Ordering::SeqCst) {
                    Self::calculate_primes(10000);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }
    }

    fn calculate_primes(limit: u64) {
        let mut sieve = vec![true; limit as usize];
        for i in 2..((limit as f64).sqrt() as u64) {
            if sieve[i as usize] {
                for j in ((i * i)..limit).step_by(i as usize) {
                    sieve[j as usize] = false;
                }
            }
        }
    }
}