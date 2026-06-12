use std::fs::{self, OpenOptions};
use std::io::{Result, Write};
use std::path::PathBuf;
use std::time::SystemTime;

pub struct Logger;

impl Logger {
    pub fn log(file_path: &str, message: &str) -> Result<()> {
        let path = PathBuf::from(file_path);

        // Ensure parent directories exist
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        // Open file for append
        let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

        let log_line = format!("{:?}: {}\n", SystemTime::now(), message);

        file.write_all(log_line.as_bytes())?;
        file.flush()
    }
}
