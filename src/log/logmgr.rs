use super::logger;
use dirs_next::data_local_dir;
pub fn log_manager(message: &str) {
    let mut path = data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    path = path.join("R-touch").join("logs").join("r-touch.log"); // home_dir/R-touch/logs/r-touch.log

    if let Err(e) = logger::Logger::log(path.to_str().unwrap(), &message) {
        eprintln!("Error logging the action. Error: {e}");
        std::process::exit(1);
    }
}
