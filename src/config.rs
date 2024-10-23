use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid directory")]
    InvalidDirectory,
    #[error("Invalid CPU load")]
    InvalidCpuLoad,
}

pub struct Config {
    pub test_dir: PathBuf,
    pub chunk_size: u64,
    pub cpu_load: u8,
}

impl Config {
    pub fn new(dir: PathBuf, chunk_size_gb: u64, cpu_load: u8) -> Result<Self, ConfigError> {
        if cpu_load > 100 {
            return Err(ConfigError::InvalidCpuLoad);
        }

        Ok(Config {
            test_dir: dir,
            chunk_size: chunk_size_gb * 1024 * 1024 * 1024,
            cpu_load,
        })
    }
}