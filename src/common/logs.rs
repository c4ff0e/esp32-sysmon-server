use simplelog::*;
use std::{
    fs::{File, create_dir_all},
    path::PathBuf,
};

use directories::ProjectDirs;

pub fn log_dir() -> Result<PathBuf, std::io::Error> {
    let project_dir = match ProjectDirs::from("com", "c4ff0e", "esp32-sysmon-server") {
        Some(project_dir) => project_dir,
        None => {
            panic!("Failed to get project directory!")
        }
    };
    let log_dir = project_dir.data_dir().to_path_buf();
    match create_dir_all(&log_dir) {
        Ok(_) => return Ok(log_dir),
        Err(e) => return Err(e),
    }
}
pub fn create_logger(log_file: &PathBuf) {
    let log_file = match File::create(log_file) {
        Ok(log_file) => log_file,
        Err(e) => {
            panic!("Failed to create log file: {}", e);
        }
    };
    let mut config_builder = ConfigBuilder::new();
    config_builder
        .set_time_level(LevelFilter::Info)
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ));

    let _ = config_builder.set_time_offset_to_local();

    let config = config_builder.build();
    match WriteLogger::init(LevelFilter::Info, config, log_file) {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to create logger: {}", e)
        }
    };
}
