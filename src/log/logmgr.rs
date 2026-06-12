use super::logger;
use dirs_next::data_local_dir;
use std::path::PathBuf;

pub fn log_manager(message: &str) {
    // OS-correct log directory
    let path: PathBuf = data_local_dir()
        .unwrap_or_else(|| PathBuf::from(".")) // fallback
        .join("R-touch")
        .join("logs")
        .join("r-touch.log");

    if let Err(e) = logger::Logger::log(path.to_str().unwrap(), message) {
        eprintln!("Error logging the action. Error: {e}");
        std::process::exit(1);
    }
}
