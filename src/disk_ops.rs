use crc32fast::Hasher;
use memmap2::MmapMut;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use thiserror::Error;
use crate::config::Config;  // Add this import

#[derive(Error, Debug)]
pub enum DiskError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Corruption detected at offset {0}")]
    Corruption(u64),
}

pub struct DiskTester {
    test_dir: PathBuf,
    chunk_size: u64,
}


impl DiskTester {
    pub fn new(config: &Config) -> Result<Self, DiskError> {
        std::fs::create_dir_all(&config.test_dir)?;
        
        Ok(Self {
            test_dir: config.test_dir.clone(),
            chunk_size: config.chunk_size,
        })
    }

    pub async fn run(&self, running: Arc<AtomicBool>) -> Result<(), DiskError> {
        while running.load(Ordering::SeqCst) {
            self.test_cycle().await?;
        }
        Ok(())
    }

    async fn test_cycle(&self) -> Result<(), DiskError> {
        let test_file = self.test_dir.join("test.bin");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&test_file)?;
            
        file.set_len(self.chunk_size)?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };

        // Generate pattern
        let mut rng = rand::thread_rng();
        let pattern: Vec<u8> = (0..1024).map(|_| rng.gen()).collect();
        let pattern_checksum = Self::calculate_checksum(&pattern);

        // Write pattern
        for chunk in mmap.chunks_mut(pattern.len()) {
            chunk.copy_from_slice(&pattern);
        }
        mmap.flush()?;

        // Verify
        for (i, chunk) in mmap.chunks(pattern.len()).enumerate() {
            if Self::calculate_checksum(chunk) != pattern_checksum {
                return Err(DiskError::Corruption(i as u64 * pattern.len() as u64));
            }
        }

        std::fs::remove_file(test_file)?;
        Ok(())
    }

    fn calculate_checksum(data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }
}
